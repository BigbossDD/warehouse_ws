import rclpy
from rclpy.node import Node
from std_msgs.msg import Float32
import random

class SensorNode(Node):

    def __init__(self):
        super().__init__('sensor_node')

        self.publisher_ = self.create_publisher(Float32, 'battery_status', 10)

        timer_period = 2.0
        self.timer = self.create_timer(timer_period, self.publish_battery)

        self.battery_level = 100.0

    def publish_battery(self):

        msg = Float32()

        # simulate battery drain
        self.battery_level -= random.uniform(0.5, 1.5)

        if self.battery_level < 0:
            self.battery_level = 100

        msg.data = self.battery_level

        self.publisher_.publish(msg)

        self.get_logger().info(f'Battery Level: {msg.data:.2f}%')


def main(args=None):

    rclpy.init(args=args)

    node = SensorNode()

    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()