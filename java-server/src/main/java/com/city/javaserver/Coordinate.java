package com.city.javaserver;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@NoArgsConstructor
public class Coordinate {
    private long timestamp;
    private double latitude;
    private double longitude;
}
