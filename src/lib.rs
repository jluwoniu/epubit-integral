use std::thread::sleep;
use std::{error::Error, time::Duration};

use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;
use thirtyfour::{prelude::WebDriverResult, By, DesiredCapabilities, WebDriver};

//const APP_NAME: &str = "epubit-integral";
const CONFIG: &str = "./default-config.toml";
//等待查找页面元素超时时间
const TIMEOUT: Duration = Duration::from_secs(10);
//等待查找点赞图标超时时间
const DIANZAN_TIMEOUT: Duration = Duration::from_secs(3);
const INTERVAL: Duration = Duration::from_millis(100);

#[derive(Serialize, Deserialize, Default, Debug)]
struct AppConfig {
    accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Account {
    username: String,
    password: String,
    page_number: usize,
}

pub fn add_account(
    username: &str,
    password: &str,
    page_number: &Option<usize>,
) -> Result<(), confy::ConfyError> {
    let mut cfg: AppConfig = confy::load_path(CONFIG)?;
    cfg.accounts.push(Account {
        username: username.to_owned(),
        password: password.to_owned(),
        page_number: page_number.unwrap_or(1),
    });
    confy::store_path(CONFIG, cfg)?;
    Ok(())
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut cfg: AppConfig = confy::load_path(CONFIG)?;
    // todo 通过命令行参数设置浏览器为chrome
    for account in &mut cfg.accounts {
        let caps = DesiredCapabilities::edge();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        login(&driver, account).await?;
        log_integral(&driver).await?;
        share_book(&driver, account).await?;
        share_course(&driver).await?;
        log_integral(&driver).await?;
        sleep(Duration::from_secs(5));
        driver.quit().await?;
    }
    println!("{:?}", cfg);
    confy::store_path(CONFIG, cfg)?;
    Ok(())
}

async fn log_integral(driver: &WebDriver) -> WebDriverResult<()> {
    driver
        .goto("https://www.epubit.com/user/sampleIndex")
        .await?;
    sleep(Duration::from_millis(500));
    let integral_ele = driver
        .query(By::ClassName("main_color"))
        .with_tag("span")
        .wait(TIMEOUT, INTERVAL)
        .first()
        .await?;
    let integral = integral_ele.text().await?;
    println!("{}", integral);
    Ok(())
}

async fn login(driver: &WebDriver, account: &Account) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/").await?;
    sleep(Duration::from_millis(1000));
    let login_button = driver
        .query(By::ClassName("login"))
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?
        .query(By::Tag("i"))
        .with_text("登录")
        .wait(TIMEOUT, INTERVAL)
        .first()
        .await?;
    login_button.wait_until().clickable().await?;
    login_button.click().await?;
    let username_input = driver
        .query(By::Id("username"))
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?;
    username_input.send_keys(&account.username).await?;
    let password_input = driver
        .query(By::Id("password"))
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?;
    password_input.send_keys(&account.password).await?;
    let login_button = driver
        .query(By::Id("passwordLoginBtn"))
        .wait(Duration::from_secs(10), Duration::from_millis(100))
        .first()
        .await?;
    sleep(Duration::from_millis(500));
    login_button.click().await?;
    Ok(())
}

//点赞并分享图书
async fn share_book(driver: &WebDriver, account: &mut Account) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/books").await?;
    sleep(Duration::from_millis(500));
    //点击下一页,直到上次运行保存的页数
    for _i in 1..account.page_number {
        driver
            .query(By::ClassName("btn-next"))
            .and_clickable()
            .wait(TIMEOUT, INTERVAL)
            .first()
            .await?
            .click()
            .await?;
    }
    //点赞图书的数量,每次点赞分享10本图书就够了
    let mut counter = 0;
    while counter < 10 {
        sleep(Duration::from_millis(1000));
        let book_list_element = driver
            .query(By::ClassName("book-list"))
            .wait(TIMEOUT, INTERVAL)
            .first()
            .await?;
        let book_list = book_list_element
            .query(By::Tag("a"))
            .wait(TIMEOUT, INTERVAL)
            .all()
            .await?;
        for book in book_list {
            book.click().await?;
            for window in driver.windows().await? {
                let original_window = driver.window().await?;
                if window != original_window {
                    driver.switch_to_window(window).await?;
                    if let Err(_) = driver
                        .query(By::ClassName("icon-dianzan"))
                        .wait(DIANZAN_TIMEOUT, INTERVAL)
                        .single()
                        .await
                    {
                        //点赞图书
                        driver
                            .find(By::ClassName("icon-dianzan1"))
                            .await?
                            .click()
                            .await?;
                        sleep(Duration::from_millis(300));
                        //分享图书
                        driver
                            .find(By::ClassName("icon-2101fenxiang"))
                            .await?
                            .click()
                            .await?;
                        counter += 1;
                        println!("点赞成功:{}", counter);
                        sleep(Duration::from_millis(1000));
                    }
                    driver.close_window().await?;
                    driver.switch_to_window(original_window).await?;
                    break;
                }
            }
            if counter >= 10 {
                println!("点赞数量完成");
                break;
            }
        }
        driver
            .query(By::ClassName("btn-next"))
            .and_clickable()
            .wait(TIMEOUT, INTERVAL)
            .first()
            .await?
            .click()
            .await?;
        account.page_number += 1;
    }
    account.page_number -= 1;
    Ok(())
}

//分享课程
async fn share_course(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/course").await?;
    sleep(Duration::from_millis(2000));
    let course_list_element = driver
        .query(By::ClassName("course-list"))
        .wait(TIMEOUT, INTERVAL)
        .first()
        .await?;
    let course_list = course_list_element.find_all(By::Tag("a")).await?;
    for course in course_list {
        course.click().await?;
        for window in driver.windows().await? {
            let original_window = driver.window().await?;
            if window != original_window {
                driver.switch_to_window(window).await?;
                let share_button = driver
                    .query(By::ClassName("icon-2101fenxiang"))
                    .and_clickable()
                    .wait(DIANZAN_TIMEOUT, INTERVAL)
                    .first()
                    .await?;
                share_button.click().await?;
                println!("分享课程成功");
                sleep(Duration::from_millis(300));
                driver.close_window().await?;
                driver.switch_to_window(original_window).await?;
                break;
            }
        }
    }
    Ok(())
}
