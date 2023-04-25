import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

Mobile.startApplication('D:\\KSWorkspace\\CodeWorkspace\\KatalonAllInOnePlatform\\NativeApp\\WikipediaSample.apk', true)

Mobile.tap(findTestObject('Object Repository/testt/android.widget.TextView - Finland joins NATO as its 31st member'), 0)

Mobile.pressBack()

Mobile.tap(findTestObject('Object Repository/testt/android.widget.TextView - Former President of the United States Donald Trump is arraigned on 34 charges of falsifying business records'), 
    0)

Mobile.tap(findTestObject('Object Repository/testt/android.widget.ImageView (1)'), 0)

Mobile.tap(findTestObject('Object Repository/testt/android.widget.TextView - Add to reading list'), 0)

Mobile.verifyElementText(findTestObject('Object Repository/testt/android.widget.TextView - GOT IT'), 'GOT IT')

Mobile.closeApplication()

