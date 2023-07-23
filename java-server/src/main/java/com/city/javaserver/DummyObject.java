package com.city.javaserver;


import lombok.*;
import org.bson.BSONObject;

import java.io.Serializable;

@Data
@AllArgsConstructor
@NoArgsConstructor
@Getter
public class DummyObject implements Serializable {
    private String id;
    private int index;
    private String guid;
    private Boolean isActive;
    private double balance;
    private String picture;
    private int age;
    private String eyeColor;
    private String name;
    private String gender;
    private String company;
    private String email;
    private String phone;
    private String address;
    private String about;
    private String registered;
    private double latitude;
    private double longitude;
    private String[] tags;
    private DummyFriend[] friends;
    private String greeting;
    private String favoriteFruit;
}
