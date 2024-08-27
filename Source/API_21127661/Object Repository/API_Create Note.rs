<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_Create Note</name>
   <tag></tag>
   <elementGuidId>0ad8c619-f5ba-4fcf-96d8-d455ab4eedb2</elementGuidId>
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
  &quot;text&quot;: &quot;{\n  \&quot;title\&quot;: \&quot;${title}\&quot;,\n  \&quot;content\&quot;: \&quot;${content}\&quot;,\n  \&quot;userIds\&quot;: [${userIds}]\n}\n&quot;,
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
      <webElementGuid>d9e38188-7d94-4a38-965a-34abf165c252</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNmE3ZjhlYi01ZjEyLTRjMWItODI3OC03NzNmYzQ3ZTEzYTgiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJmYW5oaTExMjExQGdtYWlsLmNvbSIsIm5hbWVpZGVudGlmaWVyIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyMzc5NTQwMSwiZXhwIjoxNzI2Mzg3NDAxfQ.RJnWZnYPtdZ4SYnLhYc_2JVtk-075m4SCOj0fAxZPsz90Z21L8SrWnFy-auw6k98RObTxISSN4jl2d9U5HW5vQ</value>
      <webElementGuid>436df297-a1b6-4fa7-a9dc-9a93546ae09d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/api/notes</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e5e0a52b-9ec9-4d66-b12f-21feae60c15e</id>
      <masked>false</masked>
      <name>title</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cae4c913-e5ef-4021-b71c-864671522760</id>
      <masked>false</masked>
      <name>content</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4b3b3314-2e44-4d0d-8105-cd676e7ff359</id>
      <masked>false</masked>
      <name>userIds</name>
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
