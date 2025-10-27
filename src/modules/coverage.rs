use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CoverageInfo {
    pub total_coverage: f64,
    pub files: Vec<FileCoverage>,
}

#[derive(Debug, Clone)]
pub struct FileCoverage {
    pub name: String,
    pub coverage: f64,
}

pub struct CoverageModule;

impl CoverageModule {
    pub fn get_coverage() -> Result<CoverageInfo> {
        let output = Command::new("cargo")
            .args(&["llvm-cov", "--json"])
            .output()?;
        
        if !output.status.success() {
            return Ok(CoverageInfo {
                total_coverage: 0.0,
                files: Vec::new(),
            });
        }
        
        let json_str = String::from_utf8(output.stdout)?;
        Self::parse_llvm_cov(&json_str)
    }
    
    fn parse_llvm_cov(json_str: &str) -> Result<CoverageInfo> {
        let json: serde_json::Value = serde_json::from_str(json_str)?;
        
        let mut total_covered = 0;
        let mut total_lines = 0;
        let mut files = Vec::new();
        
        if let Some(data) = json["data"].as_array() {
            for item in data {
                if let Some(functions) = item["files"].as_array() {
                    for func in functions {
                        if let Some(segments) = func["segments"].as_array() {
                            for segment in segments {
                                if segment.get(0).and_then(|v| v.as_u64()).is_some() {
                                    total_lines += 1;
                                    if let Some(hit) = segment.get(1).and_then(|v| v.as_u64()) {
                                        if hit > 0 {
                                            total_covered += 1;
                                        }
                                    }
                                }
                            }
                        }
                        
                        let filename = func["name"]
                            .as_str()
                            .unwrap_or("unknown")
                            .to_string();
                        
                        files.push(FileCoverage {
                            name: filename,
                            coverage: 0.0,
                        });
                    }
                }
            }
        }
        
        let total_coverage = if total_lines > 0 {
            (total_covered as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(CoverageInfo {
            total_coverage,
            files: files.into_iter().take(10).collect(),
        })
    }
}
