//! 列类型。

use apollo_types::Result;

/// 浮点列（A1 首切片）。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FloatColumn {
    /// 列名。
    pub name: String,
    /// 数值。可用 `NaN` 表示缺失。
    pub values: Vec<f64>,
}

impl FloatColumn {
    /// 由名称与数值构造。
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self { name: name.into(), values }
    }

    /// 行数。
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

/// 列枚举（后续扩展整数、分类、字符串等）。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Column {
    /// 浮点列。
    Float(FloatColumn),
}

impl Column {
    /// 列名。
    pub fn name(&self) -> &str {
        match self {
            Self::Float(column) => &column.name,
        }
    }

    /// 行数。
    pub fn len(&self) -> usize {
        match self {
            Self::Float(column) => column.len(),
        }
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 取浮点视图。
    pub fn as_float(&self) -> Result<&FloatColumn> {
        match self {
            Self::Float(column) => Ok(column),
        }
    }
}

impl From<FloatColumn> for Column {
    fn from(value: FloatColumn) -> Self {
        Self::Float(value)
    }
}
