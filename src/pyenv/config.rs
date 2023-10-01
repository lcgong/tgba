use anyhow::Error;

pub static PIP_VERSION: &str = "23.2.1";
pub static CPYTHON_DISTS: [(&str, &str, &str); 2] = [
    (
        "3.11.5-20230826",
        "https://gitee.com/lyucg/python-dists/releases/download/20230826/cpython-3.11.5%2020230826-x86_64-pc-windows-msvc-shared-install_only.tar.gz",
        "00f002263efc8aea896bcfaaf906b1f4dab3e5cd3db53e2b69ab9a10ba220b97"
    ),
    (
        "3.8.17-20230826",
        "https://gitee.com/lyucg/python-dists/releases/download/20230826/cpython-3.8.17%2020230826-x86_64-pc-windows-msvc-shared-install_only.tar.gz",
        "6428e1b4e0b4482d390828de7d4c82815257443416cb786abe10cb2466ca68cd"
    ),    
];


pub fn get_cpytion_candidates() -> Result<(&'static str, &'static str, &'static str), Error> {
    use super::utils::get_windows_major_versoin;
    
    let win_major = get_windows_major_versoin()?;

    if win_major > 7 {
        Ok(CPYTHON_DISTS[0])
    } else {
        Ok(CPYTHON_DISTS[1])
    }
}

pub fn get_pip_user_agent() -> String {
    // pip/23.2.1 {"ci":null,"cpu":"AMD64","implementation":{"name":"CPython","version":"3.11.4"},"installer":{"name":"pip","version":"23.2.1"},"openssl_version":"OpenSSL 1.1.1u  30 May 2023","python":"3.11.4","rustc_version":"1.72.1","setuptools_version":"65.5.0","system":{"name":"Windows","release":"10"}}
    format!("pip/{}", PIP_VERSION)
}

