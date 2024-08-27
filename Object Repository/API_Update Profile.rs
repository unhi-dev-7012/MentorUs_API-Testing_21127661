<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_Update Profile</name>
   <tag></tag>
   <elementGuidId>bc25bce7-3fe3-4197-afee-845d35963712</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNmE3ZjhlYi01ZjEyLTRjMWItODI3OC03NzNmYzQ3ZTEzYTgiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJmYW5oaTExMjExQGdtYWlsLmNvbSIsIm5hbWVpZGVudGlmaWVyIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyMzc5NTQwMSwiZXhwIjoxNzI2Mzg3NDAxfQ.RJnWZnYPtdZ4SYnLhYc_2JVtk-075m4SCOj0fAxZPsz90Z21L8SrWnFy-auw6k98RObTxISSN4jl2d9U5HW5vQ</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;${name}\&quot;,\n  \&quot;imageUrl\&quot;: \&quot;string\&quot;,\n  \&quot;phone\&quot;: \&quot;${phone}\&quot;,\n  \&quot;birthDate\&quot;: \&quot;${birthDate}\&quot;,\n  \&quot;personalEmail\&quot;: \&quot;${personalEmail}\&quot;,\n  \&quot;gender\&quot;: \&quot;${gender}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>077c062f-11c7-49e2-b324-42683813ca47</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNmE3ZjhlYi01ZjEyLTRjMWItODI3OC03NzNmYzQ3ZTEzYTgiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJmYW5oaTExMjExQGdtYWlsLmNvbSIsIm5hbWVpZGVudGlmaWVyIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyMzc5NTQwMSwiZXhwIjoxNzI2Mzg3NDAxfQ.RJnWZnYPtdZ4SYnLhYc_2JVtk-075m4SCOj0fAxZPsz90Z21L8SrWnFy-auw6k98RObTxISSN4jl2d9U5HW5vQ</value>
      <webElementGuid>316e498b-f1ac-4c9b-a1c6-ed60862990dd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>http://localhost:8080/api/users/${userId}/profile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'16a7f8eb-5f12-4c1b-8278-773fc47e13a8'</defaultValue>
      <description></description>
      <id>8c4c9960-4ad5-490e-96b7-d078d0d7bec3</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>'Fangi'</defaultValue>
      <description></description>
      <id>1d0bd9e4-278a-4485-88c0-517f8fe1c29d</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5fc1db4a-4cb8-4913-93f3-88bbb6271d1d</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>045cbeac-05c5-4d10-90b7-6a83174dbb09</id>
      <masked>false</masked>
      <name>birthDate</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>19ac8037-4e1a-43f3-907a-d5293aa26452</id>
      <masked>false</masked>
      <name>personalEmail</name>
   </variables>
   <variables>
      <defaultValue>'MALE'</defaultValue>
      <description></description>
      <id>77286507-266a-4b30-ad04-57596d1d3329</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
