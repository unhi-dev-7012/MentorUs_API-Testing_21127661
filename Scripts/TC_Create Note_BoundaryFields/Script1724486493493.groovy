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


TestObjectProperty authorizationHeader = new TestObjectProperty('Authorization', ConditionType.EQUALS, 'Bearer ' + GlobalVariable.token)


if(isTitleTest == 'true')
{
	title = generate65000CharString()
	content = 'Valid Content'
}
else
{
	title = 'Valid Title'
	content = generate65000CharString()
}



RequestObject req = findTestObject('API_Create Note', [('title') : title, ('content') : content, ('userIds') : userIds])

// Replace the existing headers or add if not present
req.getHttpHeaderProperties().removeIf({
		it.getName() == 'Authorization'
	})
req.getHttpHeaderProperties().add(authorizationHeader)

ResponseObject res = WS.sendRequest(req)

WS.verifyResponseStatusCode(res, expected.intValue(), FailureHandling.CONTINUE_ON_FAILURE)

// Verify the title in the response matches the input title
WS.verifyElementPropertyValue(res, 'title', title, FailureHandling.CONTINUE_ON_FAILURE)

// Verify the content in the response matches the input content
WS.verifyElementPropertyValue(res, 'content', content, FailureHandling.CONTINUE_ON_FAILURE)

def users = WS.getElementPropertyValue(res, 'users',  FailureHandling.CONTINUE_ON_FAILURE)

users = users.sort { user -> user.id }

List<String> userIdList = userIds.split(',').collect { it.trim().replaceAll('"', '') }.sort()

// Loop through each userId and verify it matches the corresponding user in the response
for (int i = 0; i < userIdList.size(); i++) {
	 try {
		def userId = users[i].id
		assert userId == userIdList[i] : "Expected user ID ${userIdList[i]}, but got ${userId}"
	} catch (AssertionError e) {
		// Log the failure but continue the execution
		KeywordUtil.markFailed("Assertion failed: " + e.message)
	}
}


String generate65000CharString() {
	// Create a string with 65,000 characters
	return 'a' * 65000
}


