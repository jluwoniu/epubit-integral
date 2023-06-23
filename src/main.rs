use std::time::Duration;

use thirtyfour::prelude::*;

use epubit_integral::config::update_config;
use epubit_integral::config::IntegerKey;
use toml::Value;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let page = IntegerKey("page", 1).get_value();

    //    login(&driver, "17688162304", "JLUyb441522").await?;

    login(&driver, "18820173634", "JLUep441522").await?;

    let page = share_book(&driver, page).await?;

    println!("当前页码: {}", page);
    update_config("page", Value::Integer(page)).unwrap();

    share_course(&driver).await?;

    println!("等待10秒关闭浏览器");
    std::thread::sleep(Duration::from_millis(10000));
    driver.quit().await?;
    Ok(())
}

async fn share_book(driver: &WebDriver, mut page: i64) -> WebDriverResult<i64> {
    driver.goto("https://www.epubit.com/books").await?;
    std::thread::sleep(Duration::from_millis(2000));

    for _i in 1..page {
        driver
            .find(By::ClassName("btn-next"))
            .await?
            .click()
            .await?;
        std::thread::sleep(Duration::from_millis(2000));
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
                        .wait(Duration::from_secs(3), Duration::from_secs(1))
                        .single()
                        .await
                    {
                        let dianzan1 = driver.find(By::ClassName("icon-dianzan1")).await?;
                        println!("点赞成功");
                        dianzan1.click().await?;
                        driver
                            .find(By::ClassName("icon-2101fenxiang"))
                            .await?
                            .click()
                            .await?;
                        counter += 1;
                        std::thread::sleep(Duration::from_millis(2000));
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
        std::thread::sleep(Duration::from_millis(3000));
        page += 1;
    }
    page -= 1;
    Ok(page)
}

async fn share_course(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/course").await?;
    std::thread::sleep(Duration::from_millis(2000));

    let course_list_element = driver.find(By::ClassName("course-list")).await?;
    let course_list = course_list_element.find_all(By::Tag("a")).await?;
    for course in course_list {
        course.click().await?;
        for window in driver.windows().await? {
            let original_window = driver.window().await?;
            if window != original_window {
                driver.switch_to_window(window).await?;
                if let Err(_) = driver
                    .query(By::ClassName("icon-dianzan"))
                    .wait(Duration::from_secs(3), Duration::from_secs(1))
                    .single()
                    .await
                {
                    let share_button = driver.find(By::ClassName("icon-2101fenxiang")).await?;
                    share_button.click().await?;
                    println!("分享课程成功");
                    std::thread::sleep(Duration::from_millis(1000));
                }
                driver.close_window().await?;
                driver.switch_to_window(original_window).await?;
                break;
            }
        }
    }
    Ok(())
}

async fn login(driver: &WebDriver, username: &str, password: &str) -> WebDriverResult<()> {
    driver.goto("https://www.epubit.com/").await?;
    let login_button = driver
        .find(By::XPath("//*[@id='entry']/div[1]/nav/div[2]/div[1]/i[1]"))
        .await?;
    login_button.click().await?;
    std::thread::sleep(Duration::from_millis(1000));
    let username_input = driver.find(By::Id("username")).await?;
    std::thread::sleep(Duration::from_millis(300));
    username_input.send_keys(username).await?;
    let password_input = driver.find(By::Id("password")).await?;
    std::thread::sleep(Duration::from_millis(300));
    password_input.send_keys(password).await?;
    let login_button = driver.find(By::Id("passwordLoginBtn")).await?;
    std::thread::sleep(Duration::from_millis(1000));
    login_button.click().await?;
    std::thread::sleep(Duration::from_millis(1000));
    Ok(())
}
