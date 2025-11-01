param(
  [string]$BaseUrl = "https://localhost"
)

Write-Host "[proto-smoke] BaseUrl=$BaseUrl"

function Invoke-Rpc($method, $params) {
  $body = @{ jsonrpc = '2.0'; id = [int](Get-Date -UFormat %s); method = $method; params = $params } | ConvertTo-Json -Depth 6
  $url = "$BaseUrl/proto-rpc"
  $res = curl.exe -sk -X POST -H "Content-Type: application/json" --data "$body" "$url"
  return $res
}

"-- health --"
curl.exe -sk "$BaseUrl/proto-rpc/health" | Out-Host

"-- getParams --"
Invoke-Rpc 'proto_getParams' $null | Out-Host

"-- popc_submitCommit --"
Invoke-Rpc 'popc_submitCommit' @{ payload = '0x' } | Out-Host

"-- asr_getAssignment --"
Invoke-Rpc 'asr_getAssignment' $null | Out-Host

"-- price_getCurrent --"
Invoke-Rpc 'price_getCurrent' $null | Out-Host
