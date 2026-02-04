### Kratos rust example service

- register
- login
- fake http email endpoint

### Execute

```
make infra-up
make run
```

### Available status check urls

| Сервис         | URL для браузера           | Порт на хосте | Примечание                                         |
|----------------|---------------------------|---------------|---------------------------------------------------|
| Prometheus     | http://localhost:9090      | 9090          | Веб-интерфейс, статус таргетов /targets, графики, PromQL |
| Grafana        | http://localhost:3000      | 3000          | Вход: admin/admin, дашборды для Prometheus и Loki |
| Alertmanager   | http://localhost:9093      | 9093          | Веб-интерфейс для управления алертами            |
| Node Exporter  | http://localhost:9100/metrics | 9100       | Просто метрики, UI нет                             |
| Loki           | http://localhost:3100      | 3100          | API для логов, обычно используется через Grafana |
| Promtail       | Нет веб-интерфейса         | —             | Собирает логи и отправляет в Loki                 |
