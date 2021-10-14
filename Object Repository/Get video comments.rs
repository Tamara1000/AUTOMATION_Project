<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Post a new comment on a video</description>
   <name>Get video comments</name>
   <tag></tag>
   <elementGuidId>6f4e0e92-4343-4111-b5cf-a0df5ce4ef0e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
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
   <restRequestMethod>GET</restRequestMethod>
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
      <id>fd9f0e3c-9fe6-4a26-8458-7414c7aa8ee4</id>
      <masked>false</masked>
      <name>accessToken</name>
   </variables>
   <variables>
      <defaultValue>'158830396'</defaultValue>
      <description></description>
      <id>cc15c078-24d2-44ea-ab05-0e18a52fc9fe</id>
      <masked>false</masked>
      <name>videoId</name>
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

latestCommentId = WS.getElementPropertyValue(response, &quot;data[0].resource_key&quot;)
latestCommentText = WS.getElementPropertyValue(response, &quot;data[0].text&quot;)

println(latestCommentId)
println(latestCommentText)

VariableCollections.map.putAt(&quot;latestCommentId&quot;, latestCommentId)
VariableCollections.map.putAt(&quot;latestCommentText&quot;, latestCommentText)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
