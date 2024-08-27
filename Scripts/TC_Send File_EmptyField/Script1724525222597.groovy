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
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration

TestObjectProperty authorizationHeader = new TestObjectProperty('Authorization', ConditionType.EQUALS, 'Bearer ' + GlobalVariable.token)

String projectPath = RunConfiguration.getProjectDir()
String filePath = projectPath + '\\' + file

String newId =  generateUUIDv4()
RequestObject req

switch(no)
{
	case 1:
		req = findTestObject('API_Send File', [('senderId') : GlobalVariable.myId, ('groupId') : GlobalVariable.groupId, ('file') : filePath, ('id') : ''])
		break
	case 2:
		req = findTestObject('API_Send File', [('senderId') :'', ('groupId') : GlobalVariable.groupId, ('file') : filePath, ('id') : newId])
		break
	case 3:
		req = findTestObject('API_Send File', [('senderId') : GlobalVariable.myId, ('groupId') : '' ,('file') : filePath, ('id') : newId])
		break
	default:
	break
}


// Replace the existing headers or add if not present
req.getHttpHeaderProperties().removeIf({ 
        it.getName() == 'Authorization'
    })

req.getHttpHeaderProperties().add(authorizationHeader)
String groupId = req.getVariables().get('groupId')
println("The groupId in the request body is: " + groupId)

ResponseObject res = WS.sendRequest(req)

WS.verifyResponseStatusCode(res, expected.intValue(), FailureHandling.CONTINUE_ON_FAILURE)

String responseText = res.getResponseText()

if (responseText != null && !responseText.trim().isEmpty()) {
	KeywordUtil.markPassed("Response is valid: " + responseText)
} else {
	KeywordUtil.markFailed("Response is empty or null.")
}

String generateUUIDv4() {
    return UUID.randomUUID().toString()
}

