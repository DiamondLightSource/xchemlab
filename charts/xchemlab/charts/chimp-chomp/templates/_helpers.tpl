{{/*
Expand the name of the chart.
*/}}
{{- define "chimpChomp.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "chimpChomp.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "chimpChomp.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "chimpChomp.labels" -}}
helm.sh/chart: {{ include "chimpChomp.chart" . }}
{{ include "chimpChomp.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "chimpChomp.selectorLabels" -}}
app.kubernetes.io/name: {{ include "chimpChomp.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "chimpChomp.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "xchemlab.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Create a AMQP queue name for Chimp
*/}}
{{- define "chimpChomp.queueChannel" -}}
{{- default "chimp.jobs" .Values.queue.channel }}
{{- end }}

{{/*
Create the rabbitmq host secret name
*/}}
{{- define "chimpChomp.queueHostSecretName" -}}
{{- printf "%s-%s" .Release.Name .Values.queue.host.secretNameSuffix }}
{{- end }}
