package dev.luncj.design_pattern.creational.abstract_factory;

public class SimpleShapeFactory implements Factory {
    public Shape createCurvedInstance() {
        return new Circle();
    }

    public Shape createStraightInstance() {
        return new Square();
    }
}
