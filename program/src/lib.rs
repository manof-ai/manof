use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("your_program_id");

#[program]
pub mod Manof {
    use super::*;

    pub fn initialize_agent(
        ctx: Context<InitializeAgent>,
        name: String,
        config: AgentConfig,
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;
        agent.owner = ctx.accounts.owner.key();
        agent.name = name;
        agent.config = config;
        agent.is_active = true;
        agent.performance_metrics = PerformanceMetrics::default();
        Ok(())
    }

    pub fn analyze_contract(
        ctx: Context<AnalyzeContract>,
        contract_address: Pubkey,
        analysis_params: AnalysisParams,
    ) -> Result<()> {
        let analysis = &mut ctx.accounts.contract_analysis;
        let agent = &mut ctx.accounts.agent;

        analysis.contract = contract_address;
        analysis.agent = agent.key();
        analysis.timestamp = Clock::get()?.unix_timestamp;
        analysis.params = analysis_params;
        analysis.status = AnalysisStatus::InProgress;

        // Update agent metrics
        agent.performance_metrics.total_analyses += 1;
        
        Ok(())
    }

    pub fn optimize_transaction(
        ctx: Context<OptimizeTransaction>,
        tx_data: TransactionData,
    ) -> Result<()> {
        let optimization = &mut ctx.accounts.transaction_optimization;
        let agent = &mut ctx.accounts.agent;

        optimization.transaction = tx_data;
        optimization.agent = agent.key();
        optimization.timestamp = Clock::get()?.unix_timestamp;
        optimization.status = OptimizationStatus::Processing;

        // Update agent metrics
        agent.performance_metrics.total_optimizations += 1;

        Ok(())
    }

    pub fn update_security_status(
        ctx: Context<UpdateSecurity>,
        security_data: SecurityData,
    ) -> Result<()> {
        let security = &mut ctx.accounts.security_monitor;
        let agent = &mut ctx.accounts.agent;

        security.agent = agent.key();
        security.data = security_data;
        security.last_update = Clock::get()?.unix_timestamp;

        // Update agent metrics
        if security_data.threats_detected > 0 {
            agent.performance_metrics.threats_prevented += security_data.threats_detected;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAgent<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 200 + std::mem::size_of::<AgentConfig>() + 8 + std::mem::size_of::<PerformanceMetrics>()
    )]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AnalyzeContract<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 8 + std::mem::size_of::<AnalysisParams>() + 1
    )]
    pub contract_analysis: Account<'info, ContractAnalysis>,
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OptimizeTransaction<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + std::mem::size_of::<TransactionData>() + 32 + 8 + 1
    )]
    pub transaction_optimization: Account<'info, TransactionOptimization>,
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSecurity<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + std::mem::size_of::<SecurityData>() + 8
    )]
    pub security_monitor: Account<'info, SecurityMonitor>,
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Agent {
    pub owner: Pubkey,
    pub name: String,
    pub config: AgentConfig,
    pub is_active: bool,
    pub performance_metrics: PerformanceMetrics,
}

#[account]
pub struct ContractAnalysis {
    pub contract: Pubkey,
    pub agent: Pubkey,
    pub timestamp: i64,
    pub params: AnalysisParams,
    pub status: AnalysisStatus,
}

#[account]
pub struct TransactionOptimization {
    pub transaction: TransactionData,
    pub agent: Pubkey,
    pub timestamp: i64,
    pub status: OptimizationStatus,
}

#[account]
pub struct SecurityMonitor {
    pub agent: Pubkey,
    pub data: SecurityData,
    pub last_update: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AgentConfig {
    pub analysis_threshold: u64,
    pub optimization_params: HashMap<String, u64>,
    pub security_level: SecurityLevel,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PerformanceMetrics {
    pub total_analyses: u64,
    pub total_optimizations: u64,
    pub threats_prevented: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AnalysisParams {
    pub depth: u8,
    pub include_security: bool,
    pub optimize_gas: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionData {
    pub instructions: Vec<u8>,
    pub gas_limit: u64,
    pub priority: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct SecurityData {
    pub threats_detected: u64,
    pub risk_level: RiskLevel,
    pub vulnerability_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
    Maximum,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::Medium
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum RiskLevel {
    Safe,
    Warning,
    Critical,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum AnalysisStatus {
    InProgress,
    Completed,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum OptimizationStatus {
    Processing,
    Optimized,
    Failed,
}

#[error_code]
pub enum ManofError {
    #[msg("Invalid analysis parameters")]
    InvalidAnalysisParams,
    #[msg("Invalid security level")]
    InvalidSecurityLevel,
    #[msg("Transaction optimization failed")]
    OptimizationFailed,
}
