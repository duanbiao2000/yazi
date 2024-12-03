use std::str::FromStr;

use anyhow::bail;
use serde::{Deserialize, Serialize};

// 为结构体添加克隆、复制、调试、反序列化、序列化、相等性比较的功能
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
// 将String类型转换为自定义类型
#[serde(try_from = "String")]
pub enum PreviewWrap {
	No,
	Yes,
}

impl FromStr for PreviewWrap {
	// 定义转换错误类型
	type Err = anyhow::Error;

	// 实现从字符串转换为PreviewWrap类型的函数
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// 根据字符串的值返回对应的PreviewWrap类型
		Ok(match s {
			"no" => Self::No,
			"yes" => Self::Yes,
			_ => bail!("Invalid `wrap` value: {s}"),
		})
	}
}

impl TryFrom<String> for PreviewWrap {
	// 定义转换错误类型为anyhow::Error
	type Error = anyhow::Error;

	// 实现从String类型转换为PreviewWrap类型的转换函数
	fn try_from(value: String) -> Result<Self, Self::Error> { Self::from_str(&value) }
}
