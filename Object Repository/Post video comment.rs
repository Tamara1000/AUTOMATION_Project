<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Post a new comment on a video</description>
   <name>Post video comment</name>
   <tag></tag>
   <elementGuidId>b6cc2906-ee8c-4058-b596-ce8a4d8324bc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;text\&quot;: \&quot;${commentText}\&quot;\n}&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>bearer ${accessToken}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.vimeo.com/videos/${videoId}/comments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'80fd7995b6c56cddf9a5fb8185fd6583'</defaultValue>
      <description></description>
      <id>c33677e2-27a6-4b5f-b44a-aed55660b6cc</id>
      <masked>false</masked>
      <name>accessToken</name>
   </variables>
   <variables>
      <defaultValue>'158830396'</defaultValue>
      <description></description>
      <id>acdb234c-9d12-4260-badb-0cce771c4667</id>
      <masked>false</masked>
      <name>videoId</name>
   </variables>
   <variables>
      <defaultValue>'Great video!'</defaultValue>
      <description></description>
      <id>94b105e7-961c-4269-9fac-b2a0e11bd06d</id>
      <masked>false</masked>
      <name>commentText</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import util.VariableCollections;

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)


newCommentId = WS.getElementPropertyValue(response, 'resource_key')
VariableCollections.map.putAt(&quot;newCommentId&quot;, newCommentId)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
