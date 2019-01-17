/* 
 * Kubernetes
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * OpenAPI spec version: v1.13.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IoK8sApiAutoscalingV2beta2ObjectMetricStatus : ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
  #[serde(rename = "current")]
  current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus,
  #[serde(rename = "describedObject")]
  described_object: ::models::IoK8sApiAutoscalingV2beta2CrossVersionObjectReference,
  #[serde(rename = "metric")]
  metric: ::models::IoK8sApiAutoscalingV2beta2MetricIdentifier
}

impl IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
  /// ObjectMetricStatus indicates the current value of a metric describing a kubernetes object (for example, hits-per-second on an Ingress object).
  pub fn new(current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus, described_object: ::models::IoK8sApiAutoscalingV2beta2CrossVersionObjectReference, metric: ::models::IoK8sApiAutoscalingV2beta2MetricIdentifier) -> IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
    IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
      current: current,
      described_object: described_object,
      metric: metric
    }
  }

  pub fn set_current(&mut self, current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus) {
    self.current = current;
  }

  pub fn with_current(mut self, current: ::models::IoK8sApiAutoscalingV2beta2MetricValueStatus) -> IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
    self.current = current;
    self
  }

  pub fn current(&self) -> &::models::IoK8sApiAutoscalingV2beta2MetricValueStatus {
    &self.current
  }


  pub fn set_described_object(&mut self, described_object: ::models::IoK8sApiAutoscalingV2beta2CrossVersionObjectReference) {
    self.described_object = described_object;
  }

  pub fn with_described_object(mut self, described_object: ::models::IoK8sApiAutoscalingV2beta2CrossVersionObjectReference) -> IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
    self.described_object = described_object;
    self
  }

  pub fn described_object(&self) -> &::models::IoK8sApiAutoscalingV2beta2CrossVersionObjectReference {
    &self.described_object
  }


  pub fn set_metric(&mut self, metric: ::models::IoK8sApiAutoscalingV2beta2MetricIdentifier) {
    self.metric = metric;
  }

  pub fn with_metric(mut self, metric: ::models::IoK8sApiAutoscalingV2beta2MetricIdentifier) -> IoK8sApiAutoscalingV2beta2ObjectMetricStatus {
    self.metric = metric;
    self
  }

  pub fn metric(&self) -> &::models::IoK8sApiAutoscalingV2beta2MetricIdentifier {
    &self.metric
  }


}


