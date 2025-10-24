/**
 * Axionax SDK - TypeScript Client
 * 
 * Official TypeScript SDK for interacting with Axionax Protocol
 */

import { ethers } from 'ethers';

/**
 * Job specifications for compute requests
 */
export interface JobSpecs {
  gpu: string;
  vram: number;
  framework?: string;
  region?: string;
  tags?: string[];
}

/**
 * SLA (Service Level Agreement) parameters
 */
export interface SLA {
  maxLatency: number;  // in seconds
  maxRetries: number;
  timeout: number;     // in seconds
  requiredUptime: number;  // 0.0 to 1.0
}

/**
 * Job status enum
 */
export enum JobStatus {
  Pending = 'pending',
  Assigned = 'assigned',
  Executing = 'executing',
  Committed = 'committed',
  Validating = 'validating',
  Completed = 'completed',
  Failed = 'failed',
  Slashed = 'slashed',
}

/**
 * Job information
 */
export interface Job {
  id: string;
  client: string;
  worker?: string;
  specs: JobSpecs;
  sla: SLA;
  price: bigint;
  status: JobStatus;
  submittedAt: Date;
  completedAt?: Date;
  outputRoot?: string;
}

/**
 * Worker specifications
 */
export interface WorkerSpecs {
  gpus: Array<{
    model: string;
    vram: number;
    count: number;
  }>;
  cpuCores: number;
  ram: number;
  storage: number;
  bandwidth: number;
  region: string;
}

/**
 * Worker information
 */
export interface Worker {
  address: string;
  specs: WorkerSpecs;
  reputation: number;
  stake: bigint;
  status: 'active' | 'inactive' | 'suspended' | 'slashed';
  registeredAt: Date;
}

/**
 * Axionax client configuration
 */
export interface AxionaxConfig {
  rpcUrl: string;
  chainId: number;
  privateKey?: string;
  provider?: ethers.Provider;
}

/**
 * Main Axionax SDK Client
 */
export class AxionaxClient {
  private provider: ethers.Provider;
  private signer?: ethers.Signer;
  private config: AxionaxConfig;

  constructor(config: AxionaxConfig) {
    this.config = config;
    
    if (config.provider) {
      this.provider = config.provider;
    } else {
      this.provider = new ethers.JsonRpcProvider(config.rpcUrl);
    }

    if (config.privateKey) {
      this.signer = new ethers.Wallet(config.privateKey, this.provider);
    }
  }

  /**
   * Submit a compute job
   */
  async submitJob(specs: JobSpecs, sla: SLA): Promise<Job> {
    if (!this.signer) {
      throw new Error('Signer required to submit jobs');
    }

    // TODO: Implement actual transaction
    const jobId = this.generateJobId();
    
    const job: Job = {
      id: jobId,
      client: await this.signer.getAddress(),
      specs,
      sla,
      price: BigInt(0), // TODO: Calculate from PPC
      status: JobStatus.Pending,
      submittedAt: new Date(),
    };

    return job;
  }

  /**
   * Get job status
   */
  async getJob(jobId: string): Promise<Job | null> {
    // TODO: Implement RPC call
    return null;
  }

  /**
   * List available workers
   */
  async listWorkers(filter?: Partial<WorkerSpecs>): Promise<Worker[]> {
    // TODO: Implement RPC call
    return [];
  }

  /**
   * Register as a worker
   */
  async registerWorker(specs: WorkerSpecs, stake: bigint): Promise<string> {
    if (!this.signer) {
      throw new Error('Signer required to register as worker');
    }

    // TODO: Implement actual transaction
    return await this.signer.getAddress();
  }

  /**
   * Get current price from PPC
   */
  async getCurrentPrice(jobClass: string = 'standard'): Promise<bigint> {
    // TODO: Implement RPC call to PPC
    return BigInt(1000000000000000); // 0.001 AXX
  }

  /**
   * Get network statistics
   */
  async getNetworkStats(): Promise<{
    totalWorkers: number;
    activeJobs: number;
    blockNumber: number;
    utilization: number;
  }> {
    const blockNumber = await this.provider.getBlockNumber();
    
    return {
      totalWorkers: 0,  // TODO: Implement
      activeJobs: 0,    // TODO: Implement
      blockNumber,
      utilization: 0,   // TODO: Implement
    };
  }

  /**
   * Subscribe to job status updates
   */
  onJobUpdate(jobId: string, callback: (job: Job) => void): () => void {
    // TODO: Implement WebSocket subscription
    return () => {};
  }

  private generateJobId(): string {
    return `job-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }
}

/**
 * Helper function to create Axionax client
 */
export function createClient(config: AxionaxConfig): AxionaxClient {
  return new AxionaxClient(config);
}

// Export types
export * from './types';
