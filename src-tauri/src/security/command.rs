/// 命令风险等级
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    /// 可直接执行
    Low,
    /// 需要用户确认
    High,
    /// 直接拒绝
    Blocked,
}

/// 评估命令风险等级
pub fn assess_risk(command: &str) -> RiskLevel {
    let lower = command.to_lowercase();
    let trimmed = lower.trim();

    // 黑名单：直接拒绝
    let blocked_patterns = [
        "rm -rf /",
        "rm -rf ~",
        "rm -rf $home",
        "sudo ",
        "curl | sh",
        "curl | bash",
        "wget | sh",
        "wget | bash",
        ":(){ :|:& };:",
        "mkfs",
        "dd if=/dev/zero",
        "> /dev/sda",
    ];
    for pattern in &blocked_patterns {
        if trimmed.contains(pattern) {
            return RiskLevel::Blocked;
        }
    }

    // 高危模式：需要确认
    let high_patterns = [
        "rm -rf",
        "rm -r",
        "rmdir ",
        "drop ",
        "delete from",
        "chmod -r 777",
        "mv ",
    ];
    for pattern in &high_patterns {
        if trimmed.contains(pattern) {
            return RiskLevel::High;
        }
    }

    RiskLevel::Low
}

/// 检查命令是否安全可直接执行
pub fn is_auto_executable(command: &str) -> bool {
    matches!(assess_risk(command), RiskLevel::Low)
}

/// 检查命令是否被禁止
pub fn is_blocked(command: &str) -> bool {
    matches!(assess_risk(command), RiskLevel::Blocked)
}
