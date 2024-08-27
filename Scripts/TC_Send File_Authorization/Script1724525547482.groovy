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
import com.kms.katalon.core.testobject.RequestObject as RequestObject
import com.kms.katalon.core.testobject.TestObjectProperty as TestObjectProperty
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import java.util.UUID
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration

String tokenValue = ''
if (tokenState == 'empty') {
	tokenValue = ''
} else if (tokenState == 'incorrect') {
	tokenValue = 'Bearer incorrect_token'
}
String projectPath = RunConfiguration.getProjectDir()
String filePath = projectPath + '\\' + file

TestObjectProperty authorizationHeader = new TestObjectProperty('Authorization', ConditionType.EQUALS, tokenValue)

String newId =  generateUUIDv4()

RequestObject req = findTestObject('API_Send File', [('senderId') : GlobalVariable.myId, ('groupId') : GlobalVariable.groupId, ('file') : filePath, ('id') : newId])

// Replace the existing headers or add if not present
req.getHttpHeaderProperties().removeIf({
		it.getName() == 'Authorization'
	})
req.getHttpHeaderProperties().add(authorizationHeader)

ResponseObject res = WS.sendRequest(req)

WS.verifyResponseStatusCode(res, expected.intValue(), FailureHandling.CONTINUE_ON_FAILURE)


String generateUUIDv4() {
	return UUID.randomUUID().toString()
}


