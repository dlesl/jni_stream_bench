use std::env;
use std::io;
use jni::{
    objects::{JInputStream, JOutputStream},
    InitArgsBuilder, JNIVersion, JavaVM,
};

fn main() {
    let args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .build()
        .unwrap();
    let jvm = JavaVM::new(args).unwrap();
    let env = jvm.attach_current_thread().unwrap();
    let java_in = env.get_static_field("java/lang/System", "in", "Ljava/io/InputStream;").unwrap().l().unwrap();
    let java_out = env.get_static_field("java/lang/System", "out", "Ljava/io/PrintStream;").unwrap().l().unwrap();
    let native_in = io::stdin();
    let native_out = io::stdout();
    let cmd = env::args().skip(1).next().expect("No arg");
    match cmd.as_str() {
        "java-in" => {
            let mut java_in = JInputStream::from_env(&env, java_in).unwrap();
            io::copy(&mut java_in, &mut native_out.lock())
        },
        "java-out" => {
            let mut java_out = JOutputStream::from_env(&env, java_out).unwrap();
            io::copy(&mut native_in.lock(), &mut java_out)
        },
        "native" => {
            io::copy(&mut native_in.lock(), &mut native_out.lock())
        },
        _ => unreachable!()
    }.unwrap();
}
