package dev.luncj.design_pattern.creational.abstract_factory;

public interface Factory {
    Shape createCurvedInstance();
    Shape createStraightInstance();
}
