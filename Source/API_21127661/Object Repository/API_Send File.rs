<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API_Send File</name>
   <tag></tag>
   <elementGuidId>d54858da-112b-47a4-92da-6e228456f45e</elementGuidId>
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
      &quot;name&quot;: &quot;senderId&quot;,
      &quot;value&quot;: &quot;${senderId}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;id&quot;,
      &quot;value&quot;: &quot;${id}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;file&quot;,
      &quot;value&quot;: &quot;${file}&quot;,
      &quot;type&quot;: &quot;File&quot;,
      &quot;contentType&quot;: &quot;multipart/form-data&quot;
    },
    {
      &quot;name&quot;: &quot;groupId&quot;,
      &quot;value&quot;: &quot;${groupId}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
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
      <webElementGuid>ef219ec7-8cec-4ab4-ad2f-eee96790b22d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNmE3ZjhlYi01ZjEyLTRjMWItODI3OC03NzNmYzQ3ZTEzYTgiLCJyb2xlcyI6WyJTVVBFUl9BRE1JTiJdLCJuYW1lIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJlbWFpbGFkZHJlc3MiOiJmYW5oaTExMjExQGdtYWlsLmNvbSIsIm5hbWVpZGVudGlmaWVyIjoiZmFuaGkxMTIxMUBnbWFpbC5jb20iLCJpc3MiOiJNZW50b3JVUy1sb2NhbCIsImlhdCI6MTcyMzc5NTQwMSwiZXhwIjoxNzI2Mzg3NDAxfQ.RJnWZnYPtdZ4SYnLhYc_2JVtk-075m4SCOj0fAxZPsz90Z21L8SrWnFy-auw6k98RObTxISSN4jl2d9U5HW5vQ</value>
      <webElementGuid>a3fcc76f-6da9-44ad-9fe3-f6a4303ceeb1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8080/api/messages/file</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>82b41105-ed60-496f-bfe2-62a2e6e7164f</id>
      <name>senderId</name>
      <type>AUTO_DETECT</type>
      <dataType>AUTO</dataType>
      <target>RESPONSE</target>
      <data></data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>be7c482b-fe5a-41ee-83ef-69b3baa41a98</id>
      <name>groupId</name>
      <type>AUTO_DETECT</type>
      <dataType>AUTO</dataType>
      <target>RESPONSE</target>
      <data></data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>06f194df-8c85-465f-8c5e-d0dd6b5f55cf</id>
      <name>file</name>
      <type>AUTO_DETECT</type>
      <dataType>AUTO</dataType>
      <target>RESPONSE</target>
      <data></data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>992d4030-9c42-402b-9381-0c7cab442cec</id>
      <masked>false</masked>
      <name>senderId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6e68fc3e-bfe8-47ca-bd54-c96a7897cc7a</id>
      <masked>false</masked>
      <name>groupId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>105a86f7-af66-40f7-bde6-4c606f6e5679</id>
      <masked>false</masked>
      <name>file</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e4588f4c-0090-4d70-9271-c7c218f4f256</id>
      <masked>false</masked>
      <name>id</name>
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
