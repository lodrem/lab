package dev.luncj.design_pattern.creational.abstract_factory;

public class UseCase {
    private static void draw(final Factory factory) {
        final Shape curvedShape = factory.createCurvedInstance();
        curvedShape.draw();

        final Shape straightShape = factory.createStraightInstance();
        straightShape.draw();
    }

    public static void main(String[] args) {
        final Factory simpleFactory = new SimpleShapeFactory();
        final Factory robustFactory = new RobustShapeFactory();

        draw(simpleFactory);
        draw(robustFactory);
    }
}
