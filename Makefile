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

NAME = ./target/release/kalman
SRC = ./src/client.rs   ./src/kalman.rs ./src/client.rs ./src/main.rs ./src/orchestrator.rs ./src/types.rs ./src/kalman_view.rs
IMU= ./imu-sensor-stream-macos
FEATURES = --features "implot3d"

all: $(NAME)

$(NAME): $(SRC)
	cargo build --release $(FEATURES)

run:
	cargo run $(FEATURES)

test: all
	$(IMU) --filterspeed -d 15 &
	sleep 0.1 && ./target/release/kalman

loop: all
	@failures=0; \
	for i in $$(seq 1 20); do \
	    make loop-test; \
	    exit_code=$$?; \
	    if [ $$exit_code -ne 0 ]; then \
	        failures=$$((failures + 1)); \
	    fi; \
	done; \
	echo "Command failed $$failures time(s)."

loop-test:
	$(IMU) -d 1 &
	sleep 0.1 && ./target/release/kalman

fix:
	cargo fix --bin "kalman" -p kalman $(FEATURES)

help:
	$(IMU) -h

re:
	cargo build --release

.PHONY: all build test loop
