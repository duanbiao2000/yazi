use std::str::FromStr;

// 导入anyhow库，用于处理错误
use anyhow::Context;
// 导入serde库，用于序列化和反序列化
use serde::Deserialize;
// 导入validator库，用于验证数据
use validator::Validate;

// 定义一个结构体Tasks，用于存储任务信息
#[derive(Debug, Deserialize, Validate)]
pub struct Tasks {
	// 验证micro_workers字段，不能小于1
	#[validate(range(min = 1, message = "Cannot be less than 1"))]
	pub micro_workers: u8,
	// 验证macro_workers字段，不能小于1
	#[validate(range(min = 1, message = "Cannot be less than 1"))]
	pub macro_workers: u8,
	// 验证bizarre_retry字段，不能小于1
	#[validate(range(min = 1, message = "Cannot be less than 1"))]
	pub bizarre_retry: u8,

	// 存储image_alloc字段的值
	pub image_alloc: u32,
	// 存储image_bound字段的值，为一个长度为2的u16数组
	pub image_bound: [u16; 2],

	// 存储suppress_preload字段的值，为一个bool类型
	pub suppress_preload: bool,
}

// 实现FromStr trait，用于将字符串转换为Tasks结构体
impl FromStr for Tasks {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		// 定义一个外部结构体Outer，用于存储Tasks结构体
		#[derive(Deserialize)]
		struct Outer {
			tasks: Tasks,
		}

		// 将字符串转换为Outer结构体
		let outer = toml::from_str::<Outer>(s)
			.context("Failed to parse the [tasks] section in your yazi.toml")?;
		// 验证Tasks结构体
		outer.tasks.validate()?;

		// 返回Tasks结构体
		Ok(outer.tasks)
	}
}
