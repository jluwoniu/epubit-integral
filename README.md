## 异步社区积分自动获取工具

### 使用方法

1. 构建 cargo build --release

2. 到 https://developer.microsoft.com/zh-cn/microsoft-edge/tools/webdriver/
中下载对应版本的驱动程序并运行
3. 复制default-config.toml到epubit-integral.exe同一目录下,修改配置文件的用户名和密码
3. 
```cmd
epubit-integral.exe run

```

### todo

1. 运行前后记录积分功能,暂时打印到控制台就行
2. 支持chrome浏览器功能
3. 自动下载驱动到当前目录
4. 自动运行驱动
5. 自动关闭驱动
