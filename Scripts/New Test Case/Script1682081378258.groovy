import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

Map desiredCapabilities = RunConfiguration.getDriverPreferencesProperties("WebUI")
Map prefs = desiredCapabilities.get("prefs")
String path = RunConfiguration.getProjectDir()//+"/DownloadDir/"
println(path)
prefs.put("download.default_directory", path)//"D://KSWorkspace//CodeWorkspace//KatalonAllInOnePlatform//")
RunConfiguration.setWebDriverPreferencesProperty("prefs", prefs)
WebUI.openBrowser('')
WebUI.navigateToUrl('https://7-zip.org/download.html')
WebUI.click(findTestObject('Object Repository/Page_Download/a_Download'))
//WebUI.verifyElementText(findTestObject('OR Custom Download folder/th_Description'), 'Description')
WebUI.closeBrowser()