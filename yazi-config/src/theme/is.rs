use std::str::FromStr;

use anyhow::bail;
use serde::Deserialize;
use yazi_shared::fs::Cha;

#[derive(Default, Deserialize)]
#[serde(try_from = "String")]
// 定义一个枚举类型Is，包含以下几种状态
pub enum Is {
	// 默认状态
	#[default]
	None,
	// 隐藏状态
	Hidden,
	// 链接状态
	Link,
	// 孤立状态
	Orphan,
	// 虚拟状态
	Dummy,
	// 块设备状态
	Block,
	// 字符设备状态
	Char,
	// 先进先出设备状态
	Fifo,
	// 套接字设备状态
	Sock,
	// 可执行文件状态
	Exec,
	// 粘滞位状态
	Sticky,
}

impl FromStr for Is {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"hidden" => Self::Hidden,
			"link" => Self::Link,
			"orphan" => Self::Orphan,
			"dummy" => Self::Dummy,
			"block" => Self::Block,
			"char" => Self::Char,
			"fifo" => Self::Fifo,
			"sock" => Self::Sock,
			"exec" => Self::Exec,
			"sticky" => Self::Sticky,
			_ => bail!("invalid filetype: {s}"),
		})
	}
}

impl TryFrom<String> for Is {
	type Error = anyhow::Error;

	fn try_from(s: String) -> Result<Self, Self::Error> { Self::from_str(&s) }
}

impl Is {
	#[inline]
	pub fn check(&self, cha: &Cha) -> bool {
		match self {
			Self::None => true,
			Self::Hidden => cha.is_hidden(),
			Self::Link => cha.is_link(),
			Self::Orphan => cha.is_orphan(),
			Self::Dummy => cha.is_dummy(),
			Self::Block => cha.is_block(),
			Self::Char => cha.is_char(),
			Self::Fifo => cha.is_fifo(),
			Self::Sock => cha.is_sock(),
			Self::Exec => cha.is_exec(),
			Self::Sticky => cha.is_sticky(),
		}
	}
}
