# Terminal that acts directly as a server for the rover.

import pygame
import zmq
import numpy as np
from datetime import datetime, timedelta

WIDTH=640
HEIGHT=480

pygame.init()
display = pygame.display.set_mode((WIDTH, HEIGHT))
pygame.joystick.init()
joysticks = [pygame.joystick.Joystick(x) for x in range(pygame.joystick.get_count())]
print(joysticks)
joystick = joysticks[0]

font = pygame.font.SysFont("monospace", 22)

ctx = zmq.Context()

command_socket = ctx.socket(zmq.PUB);
command_socket.bind("tcp://*:9311")

telemetry_socket = ctx.socket(zmq.SUB);
telemetry_socket.bind("tcp://*:9312")
telemetry_socket.subscribe("")

run = True
last_telemetry = {
    "status": "?",
    "timestamp": datetime.fromtimestamp(0)
}

def topleft(text):
    return (10, 10)

def bottomleft(text):
    return (10, -5-text.get_rect()[3])

def topright(text):
    return (-10-text.get_rect()[2], 10)

def draw_text(text, pos, color = "white", anchor = topleft):
    text = font.render(text, False, color)
    display.blit(text, np.array(pos) + anchor(text))

while run:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            run = False

    if telemetry_socket.poll(100, zmq.POLLIN):
        last_telemetry = telemetry_socket.recv_json(zmq.NOBLOCK)
        last_telemetry["timestamp"] = datetime.fromisoformat(last_telemetry["timestamp"][:-4])

    forward = joystick.get_axis(5) * 0.5 + 0.5
    reverse = joystick.get_axis(2) * 0.5 + 0.5
    motor = np.clip(forward - reverse, -1, 1)

    steering = joystick.get_axis(0)
    steering = np.clip(steering, -1, 1)

    command_socket.send_json({
        "motor": motor,
        "steering": steering,
    })

    # render UI
    display.fill((0,0,0))

    CENTER = (WIDTH/2, HEIGHT/2)
    RECT_RADIUS = 64
    pygame.draw.rect(display, "white", (WIDTH/2 - RECT_RADIUS, HEIGHT/2 - RECT_RADIUS, RECT_RADIUS*2, RECT_RADIUS*2), width=1)
    pygame.draw.circle(display, "white", (WIDTH/2 + RECT_RADIUS*steering, HEIGHT/2 - RECT_RADIUS*motor), 8, width=0)

    timeout = timedelta(seconds=2)
    stale_telemetry = (datetime.utcnow() - last_telemetry["timestamp"]) > timeout
    color = "red" if stale_telemetry else "white"
    draw_text(last_telemetry["status"].upper(), (0, 0), color=color)
    draw_text(last_telemetry["timestamp"].isoformat(), (0, HEIGHT), color, anchor=bottomleft)
    if stale_telemetry:
        draw_text("DISCONNECTED", (WIDTH, 0), "red", anchor=topright)
    else:
        draw_text("CONNECTED", (WIDTH, 0), "green", anchor=topright)

    pygame.display.flip()

