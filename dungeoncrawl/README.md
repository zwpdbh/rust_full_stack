# Learn 2D Game Development using 

The code is for learning book [Hands-on Rust: Effective Learning through 2D Game Development and Play](https://www.amazon.com/Hands-Rust-Effective-Learning-Development/dp/1680508164/ref=sr_1_1?crid=2K901CIU44UF5&dib=eyJ2IjoiMSJ9.l5pYSg82qxIs117kCsqdGtfpv6FgUnFVrylqhNFZO7FbJVSQ_-avvM83Pl2zK0kasBM9CkZc-y701bPq8aMmkScIwUeOCM7ej9KZG4vTvcrYGJ08EJ5B0ior06AjRUVFKBR8AGiIYM79reS3W99pIXfh1uVNxHXzu1lo5Syp5hIRDFVi7EsmrV8Dz5n4FHDdBCmdUgKd79xHAuLnuAIXDiTPOo4tY95ZA8FmbZQXTeM.5w4hv-wbq7H8uGWNaig-zkvsVcvoU60yHxBW3cyuE8A&dib_tag=se&keywords=rust+game+development&qid=1734866503&sprefix=rust+game+developme%2Caps%2C404&sr=8-1)

## Troubleshooting 

- error 

```txt 
thread 'main' panicked at core/src/panicking.rs:221:5:
unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread caused non-unwinding panic. aborting.
Aborted (core dumped)
```

- To check if your WSL2 (Windows Subsystem for Linux 2) environment supports OpenGL

```sh
sudo apt install mesa-utils
glxinfo | grep "OpenGL version"
OpenGL version string: 4.2 (Compatibility Profile) Mesa 23.2.1-1ubuntu3.1~22.04.2
```

- check if nvidia is supported in WSL2 Ubuntu 

```sh 
nvidia-smi
Sun Dec 22 20:07:39 2024       
+-----------------------------------------------------------------------------------------+
| NVIDIA-SMI 565.77.01              Driver Version: 566.36         CUDA Version: 12.7     |
|-----------------------------------------+------------------------+----------------------+
| GPU  Name                 Persistence-M | Bus-Id          Disp.A | Volatile Uncorr. ECC |
| Fan  Temp   Perf          Pwr:Usage/Cap |           Memory-Usage | GPU-Util  Compute M. |
|                                         |                        |               MIG M. |
|=========================================+========================+======================|
|   0  NVIDIA GeForce RTX 3090        On  |   00000000:01:00.0  On |                  N/A |
| 30%   28C    P8             29W /  350W |    1312MiB /  24576MiB |     13%      Default |
|                                         |                        |                  N/A |
+-----------------------------------------+------------------------+----------------------+
                                                                                         
+-----------------------------------------------------------------------------------------+
| Processes:                                                                              |
|  GPU   GI   CI        PID   Type   Process name                              GPU Memory |
|        ID   ID                                                               Usage      |
|=========================================================================================|
|    0   N/A  N/A       819      G   /Xwayland                                   N/A      |
+-----------------------------------------------------------------------------------------+
```

- Check OpenGL library 

```sh 
sudo apt install libgl1-mesa-dev libglu1-mesa-dev
```

## Reference 

- [a tutorial series on writing a Roguelike with bracket-lib](https://bfnightly.bracketproductions.com/rustbook/)
- [what is bracket-lib](https://bfnightly.bracketproductions.com/bracket-lib/what_is_it.html)