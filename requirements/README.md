

# 更新`requirements.txt`文件

* 从 https://github.com/indygreg/python-build-standalone/releases 
下载 Python独立发行包，复制到该目录下`tmp`内

* 在cmd进入该目录

* 将下载的发行版拷贝到tmp目录下，然后在`tmp`下解压出`python`目录
```sh
tar xfvz .\tmp\cpython-3.12.2+20240224-x86_64-pc-windows-msvc-shared-install_only.tar.gz -C tmp\
```

* 运行`make_requirements.bat`程序，创建虚拟环境，并自动安装`requirements.txt`所指定的包，
并在`tmp`目录下生成了`requirements.txt`文件

* 将`requirements.txt`更名为`requirements-3.12.txt`，其中3.12为Python的版本

