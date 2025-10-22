// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;
interface IERC20{function transfer(address,uint256) external returns(bool); function transferFrom(address,address,uint256) external returns(bool);}
contract AxionaxVestingVault{
    IERC20 public immutable token; address public owner;
    struct Schedule{uint64 start; uint32 cliff; uint32 duration; uint256 total; uint256 released; bool revocable;}
    mapping(address=>Schedule) public schedules;
    event Funded(address indexed b,uint256 amt); event Claimed(address indexed b,uint256 amt); event Revoked(address indexed b,uint256 refund); event OwnerChanged(address indexed o,address indexed n);
    modifier onlyOwner(){ require(msg.sender==owner,"not-owner"); _; }
    constructor(address token_){ token=IERC20(token_); owner=msg.sender; }
    function setOwner(address n) external onlyOwner{ require(n!=address(0),"zero"); emit OwnerChanged(owner,n); owner=n; }
    function fund(address b,uint64 start,uint32 cliff,uint32 dur,uint256 total,bool revocable) external onlyOwner{
        require(b!=address(0),"zero"); require(dur>0,"duration=0"); require(schedules[b].total==0,"exists");
        schedules[b]=Schedule({start:start,cliff:cliff,duration:dur,total:total,released:0,revocable:revocable});
        require(token.transferFrom(msg.sender,address(this),total),"fund-fail"); emit Funded(b,total);
    }
    function vested(address b,uint64 nowTs) public view returns(uint256){ Schedule memory s=schedules[b]; if(s.total==0) return 0; if(nowTs < s.start+uint64(s.cliff)*30 days) return 0;
        uint64 sl=s.start+uint64(s.cliff)*30 days; uint64 end=s.start+uint64(s.duration)*30 days; if(nowTs>=end) return s.total; uint64 el=nowTs-sl; uint64 ln=end-sl; return (s.total*el)/ln; }
    function claim() external{ Schedule storage s=schedules[msg.sender]; require(s.total>0,"no-sched"); uint256 v=vested(msg.sender,uint64(block.timestamp)); uint256 rel=v-s.released; require(rel>0,"nothing"); s.released+=rel; require(token.transfer(msg.sender,rel),"xfer-fail"); emit Claimed(msg.sender,rel); }
    function revoke(address b) external onlyOwner{ Schedule storage s=schedules[b]; require(s.total>0,"no-sched"); require(s.revocable,"not-revocable"); uint256 v=vested(b,uint64(block.timestamp)); uint256 ref=s.total-v; s.total=v; if(ref>0){ require(token.transfer(owner,ref),"refund-fail"); } emit Revoked(b,ref); }
}
