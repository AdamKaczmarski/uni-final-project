package com.city.javaserver;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@AllArgsConstructor
@NoArgsConstructor
public class NumericObject {
    private String _id;
    private int index;
    private int numbr;
    private Coordinate[] coordinates;
}
