#![allow(warnings)]
use std::path::PathBuf;

use clap::Parser;
use llrt_modules::{
    fs::{FsModule, FsPromisesModule},
    os::OsModule,
    path::PathModule,
    url::{init, UrlModule},
};
use rquickjs::{
    async_with,
    loader::{BuiltinResolver, ModuleLoader},
    AsyncContext, AsyncRuntime, CatchResultExt, CaughtError, Error, Function, Module,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // let runtime = Runtime::new().unwrap();
    let runtime = AsyncRuntime::new().unwrap();
    let context = AsyncContext::full(&runtime).await.unwrap();
    let loader = ModuleLoader::default()
        .with_module("path", PathModule)
        .with_module("url", UrlModule)
        .with_module("os", OsModule)
        .with_module("node:fs", FsModule)
        .with_module("node:fs/promises", FsPromisesModule);

    let resolver = BuiltinResolver::default()
        .with_module("path")
        .with_module("url")
        .with_module("os")
        .with_module("node:fs")
        .with_module("node:fs/promises");
    runtime.set_loader(resolver, loader).await;
    init_context(&context).await;

    // if use Promise，the Future will be  !Send, so it always should using async_with! macro
    // let fut = context.async_with(|ctx|{
    //     Box::pin(async move {
    //         let script = std::fs::read_to_string(&cli.file).unwrap();
    //         let rslt = Module::evaluate(ctx.clone(), "ScriptFile", script.as_bytes());
    //         if let Err(ref err) = rslt {
    //             match err {
    //                 Error::Exception => {
    //                     println!("javascript exception: {:?}", ctx.catch());
    //                 },
    //                 _ => {
    //                     println!("rquickjs error: {}", err);
    //                 },
    //             }
    //         }
    //         let promise = rslt.unwrap();
    //         promise.into_future::<()>().await?;
    //         Ok::<_,Error>(())
    //     })
    // });
    // fut.await.unwrap();

    // execute with async_with! macro
    let fut = async_with!(context =>|ctx|{
        let script = std::fs::read_to_string(&cli.file).unwrap();
        let rslt = Module::evaluate(ctx.clone(), "ScriptFile", script.as_bytes());
        if let Err(ref err) = rslt {
            match err {
                Error::Exception => {
                    println!("javascript exception: {:?}", ctx.catch());
                },
                _ => {
                    println!("rquickjs error: {}", err);
                },
            }
        }
        let promise = rslt.unwrap();
        let rslt = promise.into_future::<()>().await.catch(&ctx);
        if let Err(ref err) = rslt {
            println!("javascript error: {:?}", err);
        }
        Ok::<_,Error>(())
    });
    let _rslt = fut.await.unwrap();

    runtime.idle().await;
}

fn print(s: String) {
    println!("{s}")
}
async fn init_context(ctx: &AsyncContext) {
    let rslt = ctx.with(|ctx| -> Result<(), Error> {
        init(&ctx).unwrap();

        let global = ctx.globals();
        global.set(
            "__print",
            Function::new(ctx.clone(), print)?.with_name("__print")?,
        )?;
        ctx.eval::<(), _>(
            r#"
            globalThis.console = {
            log(...v) {
                globalThis.__print(`${v.join(" ")}`)
            },
            error(...v) {
                globalThis.__print(`${v.join(" ")}`)
            }

            }
            "#,
        )?;
        Ok(())
    });
    rslt.await.unwrap();
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// script path
    file: PathBuf,
}
