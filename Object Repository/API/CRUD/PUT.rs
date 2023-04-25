<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT</name>
   <tag></tag>
   <elementGuidId>0272bf00-9ba6-4464-90da-128c5eecedfe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;userId\&quot;: \&quot;string\&quot;,\n  \&quot;username\&quot;: \&quot;string\&quot;,\n  \&quot;books\&quot;: [\n    {\n      \&quot;isbn\&quot;:\&quot;9781449365035\&quot;,\n      \&quot;title\&quot;:\&quot;Speaking JavaScript\&quot;,\n      \&quot;subTitle\&quot;:\&quot;An In-Depth Guide for Programmers\&quot;,\n      \&quot;author\&quot;:\&quot;Axel Rauschmayer\&quot;,\n      \&quot;publish_date\&quot;:\&quot;2014-02-01T00:00:00.000Z\&quot;,\n      \&quot;publisher\&quot;:\&quot;O\u0027Reilly Media - put\&quot;,\n      \&quot;pages\&quot;:460,\n      \&quot;description\&quot;:\&quot;Like it or not, JavaScript is everywhere these days-from browser to server to mobile-and now you, too, need to learn the language or dive deeper than you have. This concise book guides you into and through JavaScript, written by a veteran programmer who o\&quot;,\n      \&quot;website\&quot;:\&quot;http://speakingjs.com/\&quot;\n    }\n  ]\n}&quot;,
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
      <webElementGuid>a0545276-495a-43e2-b8fa-8ef0ef697ae6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://demoqa.com/BookStore/v1/Books/9781449365035</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
