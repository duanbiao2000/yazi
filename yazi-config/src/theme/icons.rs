use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Deserializer};
use yazi_shared::{Condition, fs::File, theme::{Color, Icon, Style}};

use crate::{Pattern, Preset};

// 定义一个Icons结构体，包含globs、dirs、files、exts和conds五个成员变量
pub struct Icons {
	// 存储模式与图标的向量
	globs: Vec<(Pattern, Icon)>,
	// 存储目录与图标的哈希表
	dirs:  HashMap<String, Icon>,
	// 存储文件与图标的哈希表
	files: HashMap<String, Icon>,
	// 存储扩展名与图标的哈希表
	exts:  HashMap<String, Icon>,
	// 定义一个向量，其中包含条件（Condition）和图标（Icon）的元组
	conds: Vec<(Condition, Icon)>,
}

impl Icons {
	pub fn matches(&self, file: &File) -> Option<&Icon> {
		if let Some(i) = self.match_by_glob(file) {
			return Some(i);
		}

		if let Some(i) = self.match_by_name(file) {
			return Some(i);
		}

		// 定义一个闭包，用于判断文件类型
		let f = |s: &str| match s {
			// 如果字符串为"dir"，则判断文件是否为目录
			"dir" => file.is_dir(),
			// 如果字符串为"hidden"，则判断文件是否为隐藏文件
			"hidden" => file.is_hidden(),
			// 如果字符串为"link"，则判断文件是否为链接
			"link" => file.is_link(),
			// 如果字符串为"orphan"，则判断文件是否为孤立文件
			"orphan" => file.is_orphan(),
			// 如果字符串为"dummy"，则判断文件是否为虚拟文件
			"dummy" => file.is_dummy(),
			// 如果字符串为"block"，则判断文件是否为块设备文件
			"block" => file.is_block(),
			// 如果字符串为"char"，则判断文件是否为字符设备文件
			"char" => file.is_char(),
			// 如果字符串为"fifo"，则判断文件是否为命名管道文件
			"fifo" => file.is_fifo(),
			// 如果字符串为"sock"，则判断文件是否为套接字文件
			"sock" => file.is_sock(),
			// 如果字符串为"exec"，则判断文件是否为可执行文件
			"exec" => file.is_exec(),
			// 如果字符串为"sticky"，则判断文件是否为粘滞位文件
			"sticky" => file.is_sticky(),
			// 如果字符串不匹配以上任何一种情况，则返回false
			_ => false,
		};
		self.conds.iter().find(|(c, _)| c.eval(f) == Some(true)).map(|(_, i)| i)
	}

	#[inline]
	fn match_by_glob(&self, file: &File) -> Option<&Icon> {
		self.globs.iter().find(|(p, _)| p.match_path(&file.url, file.is_dir())).map(|(_, i)| i)
	}

	#[inline]
	fn match_by_name(&self, file: &File) -> Option<&Icon> {
		let name = file.name().to_str()?;
		if file.is_dir() {
			self.dirs.get(name).or_else(|| self.dirs.get(&name.to_ascii_lowercase()))
		} else {
			self
				.files
				.get(name)
				.or_else(|| self.files.get(&name.to_ascii_lowercase()))
				.or_else(|| self.match_by_ext(file))
		}
	}

	#[inline]
	fn match_by_ext(&self, file: &File) -> Option<&Icon> {
		let ext = file.url.extension()?.to_str()?;
		self.exts.get(ext).or_else(|| self.exts.get(&ext.to_ascii_lowercase()))
	}
}

impl<'de> Deserialize<'de> for Icons {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		pub struct Shadow {
			globs:         Vec<ShadowPat>,
			#[serde(default)]
			prepend_globs: Vec<ShadowPat>,
			#[serde(default)]
			append_globs:  Vec<ShadowPat>,

			dirs:         Vec<ShadowStr>,
			#[serde(default)]
			prepend_dirs: Vec<ShadowStr>,
			#[serde(default)]
			append_dirs:  Vec<ShadowStr>,

			files:         Vec<ShadowStr>,
			#[serde(default)]
			prepend_files: Vec<ShadowStr>,
			#[serde(default)]
			append_files:  Vec<ShadowStr>,

			exts:         Vec<ShadowStr>,
			#[serde(default)]
			prepend_exts: Vec<ShadowStr>,
			#[serde(default)]
			append_exts:  Vec<ShadowStr>,

			conds:         Vec<ShadowCond>,
			#[serde(default)]
			prepend_conds: Vec<ShadowCond>,
			#[serde(default)]
			append_conds:  Vec<ShadowCond>,
		}
		#[derive(Deserialize)]
		pub struct ShadowPat {
			name: Pattern,
			text: String,
			fg:   Option<Color>,
		}
		#[derive(Deserialize)]
		pub struct ShadowStr {
			name: String,
			text: String,
			fg:   Option<Color>,
		}
		#[derive(Deserialize)]
		pub struct ShadowCond {
			#[serde(rename = "if")]
			if_:  Condition,
			text: String,
			fg:   Option<Color>,
		}

		let shadow = Shadow::deserialize(deserializer)?;

		let globs = Preset::mix(shadow.prepend_globs, shadow.globs, shadow.append_globs)
			.map(|v| (v.name, Icon { text: v.text, style: Style { fg: v.fg, ..Default::default() } }))
			.collect();

		let conds = Preset::mix(shadow.prepend_conds, shadow.conds, shadow.append_conds)
			.map(|v| (v.if_, Icon { text: v.text, style: Style { fg: v.fg, ..Default::default() } }))
			.collect();

		fn as_map(it: impl Iterator<Item = ShadowStr>) -> HashMap<String, Icon> {
			let mut map = HashMap::with_capacity(it.size_hint().0);
			for v in it {
				map
					.entry(v.name)
					.or_insert(Icon { text: v.text, style: Style { fg: v.fg, ..Default::default() } });
			}
			map.shrink_to_fit();
			map
		}

		Ok(Self {
			globs,
			dirs: as_map(Preset::mix(shadow.prepend_dirs, shadow.dirs, shadow.append_dirs)),
			files: as_map(Preset::mix(shadow.prepend_files, shadow.files, shadow.append_files)),
			exts: as_map(Preset::mix(shadow.prepend_exts, shadow.exts, shadow.append_exts)),
			conds,
		})
	}
}
