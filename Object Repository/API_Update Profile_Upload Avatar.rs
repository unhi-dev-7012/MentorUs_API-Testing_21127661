<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_Update Profile_Upload Avatar</name>
   <tag></tag>
   <elementGuidId>00c4cf64-5d9f-4c99-9501-63fd6c4dda0f</elementGuidId>
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
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;file&quot;,
      &quot;value&quot;: &quot;${file}&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>cf11ea12-018d-40c6-94c4-76019696cc42</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNmE3ZjhlYi01ZjEyLTRjMWItODI3OC03NzNmYzQ3ZTEzYTgiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJmYW5oaTExMjExQGdtYWlsLmNvbSIsIm5hbWVpZGVudGlmaWVyIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyMzc5NTQwMSwiZXhwIjoxNzI2Mzg3NDAxfQ.RJnWZnYPtdZ4SYnLhYc_2JVtk-075m4SCOj0fAxZPsz90Z21L8SrWnFy-auw6k98RObTxISSN4jl2d9U5HW5vQ</value>
      <webElementGuid>8b79f869-3f49-4ce0-b46e-ad3977b3b628</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/api/users/avatar</restUrl>
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
      <id>065cee74-e340-4c1b-8c97-f9e5aebee80c</id>
      <masked>false</masked>
      <name>file</name>
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
