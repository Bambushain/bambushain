// Uses Declarative syntax to run commands inside a container.
pipeline {
    triggers {
        pollSCM("*/5 * * * *")
    }
    agent {
        kubernetes {
            yaml '''
apiVersion: v1
kind: Pod
spec:
  volumes:
    - name: docker-sock
      hostPath:
        path: /var/run/docker.sock
  containers:
  - name: docker
    image: quay.imanuel.dev/dockerhub/library---docker:stable
    command:
    - cat
    tty: true
    volumeMounts:
    - mountPath: /var/run/docker.sock
      name: docker-sock
'''
        }
    }
    stages {
        stage('Build container') {
            when {
              branch 'main'
            }
            steps {
                parallel {
                    stage('App') {
                        steps {
                            container('docker') {
                                sh "docker build -t quay.imanuel.dev/creastina/sheef-rs:$BUILD_NUMBER -f ./Dockerfile ."
                                sh "docker tag quay.imanuel.dev/creastina/sheef-rs:$BUILD_NUMBER quay.imanuel.dev/creastina/sheef-rs:latest"

                                withDockerRegistry(credentialsId: 'quay.imanuel.dev', url: 'https://quay.imanuel.dev') {
                                    sh "docker push quay.imanuel.dev/creastina/sheef-rs:$BUILD_NUMBER"
                                    sh "docker push quay.imanuel.dev/creastina/sheef-rs:latest"
                                }
                            }
                        }
                    }
                    stage('Docs') {
                        steps {
                            container('docker') {
                                sh "docker build -t quay.imanuel.dev/creastina/sheef-api-docs:$BUILD_NUMBER -f ./api_docs/Dockerfile ."
                                sh "docker tag quay.imanuel.dev/creastina/sheef-api-docs:$BUILD_NUMBER quay.imanuel.dev/creastina/sheef-api-docs:latest"

                                withDockerRegistry(credentialsId: 'quay.imanuel.dev', url: 'https://quay.imanuel.dev') {
                                    sh "docker push quay.imanuel.dev/creastina/sheef-api-docs:$BUILD_NUMBER"
                                    sh "docker push quay.imanuel.dev/creastina/sheef-api-docs:latest"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
