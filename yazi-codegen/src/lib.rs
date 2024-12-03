use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn};

// 定义一个名为command的proc_macro_attribute宏
#[proc_macro_attribute]
pub fn command(_: TokenStream, item: TokenStream) -> TokenStream {
	// 将传入的item解析为ItemFn类型
	let mut f: ItemFn = syn::parse(item).unwrap();
	let mut ins = f.sig.inputs.clone();

	// Turn `opt: Opt` into `opt: impl Into<Opt>`
	ins[1] = {
		// 检查函数签名中的第二个参数是否为Typed类型
		let FnArg::Typed(opt) = &f.sig.inputs[1] else {
			// 如果不是，则抛出异常
			panic!("Cannot find the `opt` argument in the function signature.");
		};

		// 获取参数的类型
		let opt_ty = &opt.ty;
		// 解析参数类型为impl Into<#opt_ty>
		syn::parse2(quote! { opt: impl Into<#opt_ty> }).unwrap()
	};

	// Make the original function private and add a public wrapper
	// 断言f的可见性为公共
	assert!(matches!(f.vis, syn::Visibility::Public(_)));
	// 将f的可见性设置为继承
	f.vis = syn::Visibility::Inherited;

	// Add `__` prefix to the original function name
	// 获取函数的名称
	let name_ori = f.sig.ident;
	// 将函数名称前加上"__"前缀
	f.sig.ident = format_ident!("__{}", name_ori);
	// 获取新的函数名称
	let name_new = &f.sig.ident;

	// Collect the rest of the arguments
	let rest_args = ins.iter().skip(2).map(|arg| match arg {
		FnArg::Receiver(_) => unreachable!(),
		FnArg::Typed(t) => &t.pat,
	});

	quote! {
		// 定义一个内联函数，函数名为name_ori，参数为ins
		#[inline]
		pub fn #name_ori(#ins) { self.#name_new(opt.into(), #(#rest_args),*); }
		// 将f转换为字符串
		#f
	}
	.into()
}
