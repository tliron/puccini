use k8s_openapi::{
    api::{apps::v1::*, core::v1::*},
    apimachinery::pkg::{api::resource::*, apis::meta::v1::*, util::intstr::*},
};

pub fn new_deployment(label: &str, name: &str) -> Deployment {
    Deployment {
        metadata: ObjectMeta {
            name: Some(name.into()),
            labels: Some([(label.into(), name.into())].into()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            selector: LabelSelector { match_labels: Some([(label.into(), name.into())].into()), ..Default::default() },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta { labels: Some([(label.into(), name.into())].into()), ..Default::default() }),
                spec: Some(PodSpec {
                    service_account_name: Some(name.into()),
                    termination_grace_period_seconds: Some(5),
                    security_context: Some(PodSecurityContext {
                        fs_group: Some(1000),
                        run_as_group: Some(1000),
                        run_as_non_root: Some(true),
                        run_as_user: Some(1000),
                        ..Default::default()
                    }),
                    containers: vec![Container {
                        name: "server".into(),
                        security_context: Some(SecurityContext {
                            allow_privilege_escalation: Some(false),
                            capabilities: Some(Capabilities { drop: Some(vec!["ALL".into()]), ..Default::default() }),
                            privileged: Some(false),
                            read_only_root_filesystem: Some(true),
                            ..Default::default()
                        }),
                        image: Some(name.into()),
                        ports: Some(vec![ContainerPort { container_port: 9555, ..Default::default() }]),
                        env: Some(vec![EnvVar {
                            name: "PORT".into(),
                            value: Some("9555".into()),
                            ..Default::default()
                        }]),
                        resources: Some(ResourceRequirements {
                            requests: Some(
                                [("cpu".into(), Quantity("200m".into())), ("memory".into(), Quantity("180Mi".into()))]
                                    .into(),
                            ),
                            limits: Some(
                                [("cpu".into(), Quantity("300m".into())), ("memory".into(), Quantity("300Mi".into()))]
                                    .into(),
                            ),
                            ..Default::default()
                        }),
                        readiness_probe: Some(Probe {
                            initial_delay_seconds: Some(20),
                            period_seconds: Some(15),
                            grpc: Some(GRPCAction { port: 9555, ..Default::default() }),
                            ..Default::default()
                        }),
                        liveness_probe: Some(Probe {
                            initial_delay_seconds: Some(20),
                            period_seconds: Some(15),
                            grpc: Some(GRPCAction { port: 9555, ..Default::default() }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn _new_service(label: &str, name: &str) -> Service {
    Service {
        metadata: ObjectMeta {
            name: Some(name.into()),
            labels: Some([(label.into(), name.into())].into()),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some("ClusterIP".into()),
            selector: Some([(label.into(), name.into())].into()),
            ports: Some(vec![ServicePort {
                name: Some("grpc".into()),
                port: 9555,
                target_port: Some(IntOrString::Int(9555)),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn _new_service_account(name: &str) -> ServiceAccount {
    ServiceAccount { metadata: ObjectMeta { name: Some(name.into()), ..Default::default() }, ..Default::default() }
}
