import asyncio
from playwright.async_api import async_playwright

async def main():
    async with async_playwright() as p:
        browser = await p.chromium.launch()
        page = await browser.new_page()
        # Construct the file path relative to the current working directory
        import os
        file_path = "file://" + os.path.abspath("docs/index.html")
        await page.goto(file_path)
        await page.screenshot(path="screenshot.png")
        await browser.close()

if __name__ == "__main__":
    asyncio.run(main())
