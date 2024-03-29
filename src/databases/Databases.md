# Архитектура приложения
## Приложение №1
### Логика работы
- .
### Базы данных
#### ClickHouse
- Храним данные для аналитики
## Приложение №2
### Логика работы
- Выдаёт дополнительную информацию о пользователе:
  - Место работы
  - Место обучения в школе
  - Место обучения в колледже/университете
  - ...
- Общение с приложением №1 через кафку
### Базы данных
#### Mongo
- Храним все связи 1 к 1 (Пример: один пользователь имеет один адрес проживания)
  - См. [Логика работы](#Логика_работы)
## Приложение №3
### Логика работы
- Регистрирует пользователя
- Создаёт посты от имени пользователя
- Хранит токен авторизации
### Базы данных
#### Sqlite
- Храним все связи 1 к многим (Пример: один пользователь имеет много постов)
  - Пользователи (id, имя, фамилия, отчество (опционально))
  - Посты (id, user_id, название, текст, время создания)
#### Redis
- Храним инфу, которая постоянно требуется приложением (Высокая скорость r/w)
  - Токен авторизации
## Приложение №4
### Логика работы
- Создаёт подписки
- Подписывается один пользователь на другого
### Базы данных
#### Neo4j
- Храним все связи многие ко многим (Пример: многие пользователи могут иметь многие подписки)
  - Вершины - пользователи
  - Грани (с направлением) - подписки
