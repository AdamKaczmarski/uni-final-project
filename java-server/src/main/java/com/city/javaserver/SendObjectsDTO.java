package com.city.javaserver;

import lombok.Data;

@Data
public class SendObjectsDTO {
    private int  amount;
    private String providerUrl;
    private ObjectFormat objectFormat;
    private String serverType;
    private ObjectType objectType;
}
