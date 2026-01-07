# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: ale-tell <ale-tell@42student.fr>           +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2025/03/15 18:21:11 by ale-tell          #+#    #+#              #
#    Updated: 2025/03/15 18:21:11 by ale-tell         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = ./target/$(TYPE)/kalman
SRC =	./src/client.rs \
		./src/kalman.rs \
		./src/client.rs \
		./src/main.rs \
		./src/orchestrator.rs \
		./src/types.rs \
		./src/log.rs \
		./src/gui.rs \
		./src/message.rs \
		./src/lib.rs \
		./src/plot_data.rs

IMU= ./imu-sensor-stream-macos

all: $(NAME)

$(NAME): $(SRC)
	cargo build --release

test: all
	$(IMU) --filterspeed &
	sleep 0.1 && ./target/release/kalman

test-gui: all
	# $(IMU) --filterspeed -s 4 -d 5 -n 2.7 & # show scatter
	# $(IMU) --filterspeed -s 2 -n 2.4 & # high delta
	$(IMU) --filterspeed  &
	sleep 0.1 && ./target/release/kalman --gui


fix:
	cargo fix --bin "kalman" -p kalman

help:
	$(IMU) -h

.PHONY: all build test loop
