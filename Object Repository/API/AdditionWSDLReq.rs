<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AdditionWSDLReq</name>
   <tag></tag>
   <elementGuidId>fa2e109b-6ca3-48d3-9669-68525c360061</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>51daf629-71d9-4484-ac79-6aa529bb8a09</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://www.w3.org/2003/05/soap-envelope&quot; xmlns:tem=&quot;http://tempuri.org/&quot;>
   &lt;soap:Header/>
   &lt;soap:Body>
      &lt;tem:Add>
         &lt;tem:a>13&lt;/tem:a>
         &lt;tem:b>13&lt;/tem:b>
      &lt;/tem:Add>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceEndpoint>https://ecs.syr.edu/faculty/fawcett/Handouts/cse775/code/calcWebService/Calc.asmx</soapServiceEndpoint>
   <soapServiceFunction>Add</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import groovy.util.slurpersupport.GPathResult

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

String body = response.getResponseText()
WS.comment(&quot;body is &quot; + body)

GPathResult parsed = new XmlSlurper().parseText(body)
def status = parsed.getProperty(&quot;//AddResponse@xmlns&quot;)

//def attr1=response.getBodyContent(&quot;//*:AddResponse/@xmlns&quot;)
println &quot;Value of attr1 is &quot; + status

//String status = parsed.Body.getElementValue()
//WS.comment(&quot;Status is &quot; + status)
//println 'resu'+status
//WS.verifyElementText(response, 'AddResponse', '>')


assertThat(response.getResponseText()).contains('http://www.w3.org/2001/XMLSchema')</verificationScript>
   <wsdlAddress>https://ecs.syr.edu/faculty/fawcett/Handouts/cse775/code/calcWebService/Calc.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
