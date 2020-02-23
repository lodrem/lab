package dev.luncj.design_pattern.creational.abstract_factory;

public class RobustShapeFactory implements Factory {
    public Shape createCurvedInstance() {
        return new Ellipse();
    }
    public Shape createStraightInstance() {
        return new Rectangle();
    }
}
