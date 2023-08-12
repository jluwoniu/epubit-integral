use std::thread::sleep;
use std::{error::Error, time::Duration};

use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;
use thirtyfour::{prelude::WebDriverResult, By, DesiredCapabilities, WebDriver};


//const APP_NAME: &str = "epubit-integral";
const CONFIG: &str = "./default-config.toml";

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

    // todo 通过命令行参数设置浏览器为edge

    for account in &mut cfg.accounts {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:9515", caps).await?;
        login(&driver, account).await?;
        share_book(&driver, account).await?;
        share_course(&driver).await?;
        driver.quit().await?;
    }
    println!("{:?}", cfg);
    confy::store_path(CONFIG, cfg)?;
    Ok(())
}

async fn login(driver: &WebDriver, account: &Account) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/").await?;
    let login_button = driver
        .find(By::XPath("//*[@id='entry']/div[1]/nav/div[2]/div[1]/i[1]"))
        .await?;
    login_button.click().await?;
    sleep(Duration::from_millis(2000));
    let username_input = driver.find(By::Id("username")).await?;
    sleep(Duration::from_millis(1000));
    username_input.send_keys(&account.username).await?;
    let password_input = driver.find(By::Id("password")).await?;
    sleep(Duration::from_millis(1000));
    password_input.send_keys(&account.password).await?;
    let login_button = driver.find(By::Id("passwordLoginBtn")).await?;
    sleep(Duration::from_millis(500));
    login_button.click().await?;
    Ok(())
}

async fn share_book(driver: &WebDriver, account: &mut Account) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/books").await?;
    sleep(Duration::from_millis(300));

    for _i in 1..account.page_number {
        driver
            .find(By::ClassName("btn-next"))
            .await?
            .click()
            .await?;
        sleep(Duration::from_millis(1500));
    }

    let mut counter = 0;

    while counter < 10 {
        let book_list_element = driver.find(By::ClassName("book-list")).await?;
        let book_list = book_list_element.find_all(By::Tag("a")).await?;
        for book in book_list {
            book.click().await?;
            for window in driver.windows().await? {
                let original_window = driver.window().await?;
                if window != original_window {
                    driver.switch_to_window(window).await?;
                    if let Err(_) = driver
                        .query(By::ClassName("icon-dianzan"))
                        .wait(Duration::from_millis(2500), Duration::from_millis(300))
                        .single()
                        .await
                    {
                        let dianzan1 = driver.find(By::ClassName("icon-dianzan1")).await?;
                        dianzan1.click().await?;
                        driver
                            .find(By::ClassName("icon-2101fenxiang"))
                            .await?
                            .click()
                            .await?;
                        counter += 1;
                        println!("点赞成功:{}", counter);
                        sleep(Duration::from_millis(300));
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
            .find(By::ClassName("btn-next"))
            .await?
            .click()
            .await?;
        sleep(Duration::from_millis(1500));
        account.page_number += 1;
    }
    account.page_number -= 1;
    Ok(())
}

async fn share_course(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/course").await?;
    sleep(Duration::from_millis(500));
    let course_list_element = driver.find(By::ClassName("course-list")).await?;
    let course_list = course_list_element.find_all(By::Tag("a")).await?;
    for course in course_list {
        course.click().await?;
        for window in driver.windows().await? {
            let original_window = driver.window().await?;
            if window != original_window {
                driver.switch_to_window(window).await?;
                let share_button = driver.find(By::ClassName("icon-2101fenxiang")).await?;
                sleep(Duration::from_millis(500));
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
