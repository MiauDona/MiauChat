# MiauChat
Soporte de chat en tiempo real usando WebSockets.

## Características:
- Chats uno a uno o en grupo.
- Notificaciones de nuevos mensajes.
- Enviar y recibir mensajes en tiempo real.
- Soporte para emojis, imágenes y archivos.

## Requisitos
- Se tiene que abrir una conexion con un usuario a traves de un nombre, mantener la conexion abierta hasta que se cierre la app
- Se tiene que poder enviar y recibr mensajes a traves de la conexion
- Se tiene que poder ver los mensajes enviados y recibidos
- Se tiene que poder cerrar la conexion
- Se tienen que mantener y cambiar las configuraciones de la conexion y del usuario
- Se tiene que poder ver el estado de la conexion tuyo y de otros usuarios
- Se tiene que poder ver la configuracion del usuario
- Se tienen que guardar los usuarios conectados anteriormente contigo
- Si el nombre del usuario que debe de ser unico ya esta en uso, se tiene que mostrar un mensaje de error, no permitir la conexion y asignar valores por defecto unicos
- Tiene que llegar una notificacion a windows cuando se reciba un mensaje
- Se tienen que poder enviar archivos y probablemente emojis
- Las imágenes tal vez deben mostrarse como imagen en lugar de como archivo en el chat