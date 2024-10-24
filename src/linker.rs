use std::path::{Path, PathBuf};

pub fn link_binary(objects: &[PathBuf], output_filename: &Path) -> std::io::Result<()> {
    let objects: Vec<_> = objects.iter().map(|x| x.display().to_string()).collect();
    let output_filename = output_filename.to_string_lossy().to_string();

    let args: Vec<_> = {
        let (scrt1, crti, crtn) = {
            if file_exists("/usr/lib64/Scrt1.o") {
                (
                    "/usr/lib64/Scrt1.o",
                    "/usr/lib64/crti.o",
                    "/usr/lib64/crtn.o",
                )
            } else {
                (
                    "/lib/x86_64-linux-gnu/Scrt1.o",
                    "/lib/x86_64-linux-gnu/crti.o",
                    "/lib/x86_64-linux-gnu/crtn.o",
                )
            }
        };

        let mut args = vec![
            "-pie",
            "--hash-style=gnu",
            "--eh-frame-hdr",
            "--dynamic-linker",
            "/lib64/ld-linux-x86-64.so.2",
            "-m",
            "elf_x86_64",
            scrt1,
            crti,
        ];

        args.extend(&["-o", &output_filename]);

        args.extend(&[
            "-L/lib64",
            "-L/usr/lib64",
            "-L/lib/x86_64-linux-gnu",
            "-zrelro",
            "--no-as-needed",
            "-lc",
            "-O1",
            crtn,
        ]);

        args.extend(objects.iter().map(|x| x.as_str()));

        args
    };

    let mut linker = std::process::Command::new("ld");
    let proc = linker.args(args.iter()).spawn()?;
    let _output = proc.wait_with_output()?;
    Ok(())
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}
