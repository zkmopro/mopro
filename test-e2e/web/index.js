const { Builder, By } = require('selenium-webdriver');
const chrome = require('selenium-webdriver/chrome');

(async function testStatusCheck() {
    // Configure chrome if env set
    const options = new chrome.Options();
    if (process.env.CHROME_BIN) {
        options.setChromeBinaryPath(process.env.CHROME_BIN);
    }
    options.addArguments('--headless');
    options.addArguments('--no-sandbox');

    const driverBuilder = new Builder()
    .forBrowser('chrome')
    .setChromeOptions(options);
    
    // Configure chromewdriver if env set
    if (process.env.CHROMEDRIVER_BIN) {
        const service = new chrome.ServiceBuilder(process.env.CHROMEDRIVER_BIN);
        driverBuilder.setChromeService(service);
    }
    
    try {
        const driver = await driverBuilder.build();
        
        // Log ChromeDriver version via WebDriver
        const driverVersion = await driver.executeScript('return navigator.userAgent');
        console.log(`WebDriver user agent: ${driverVersion}`);

        await driver.get('http://localhost:3000');

        // Wait for the test completion marker
        const statusDiv = await driver.findElement(By.id('test-status'));
        await driver.wait(async () => {
            const status = await statusDiv.getAttribute('data-status');
            return status === 'passed' || status === 'failed';
        }, 10000);

        // Check the final test status
        const finalStatus = await statusDiv.getAttribute('data-status');

        if (finalStatus === 'passed') {
            console.log("All tests passed!");
            process.exit(0);
        } else {
            console.log("Some test(s) failed");
            process.exit(1);
        }
    } catch (error) {
        console.error("Error during test:", error);
        process.exit(1);
    } finally {
        await driver.quit();
    }
})();
