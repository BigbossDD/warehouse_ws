import rclpy #
from rclpy.node import Node # 
from std_msgs.msg import Float32 #
import random
import time

class SensorNode(Node):

    def __init__(self):
        super().__init__('sensor_node')
        timer = 2.0
        
        self.publisher = self.create_publisher(Float32, 'battery_status', 10) # type / name   / buffer size 

        
        self.timer = self.create_timer(timer, self.publish_battery) # how freq to  msg 
        
        self.battery_lvl = 100.0
    def publish_battery(self): # 

        msg= Float32()

        # this will show that the battry lvl going down as if robot is used 
        self.battery_lvl -= random.uniform(0.5, 1.5)

        if self.battery_lvl < 0:
            self.battery_lvl = 100

        msg.data = self.battery_lvl

        self.publisher.publish(msg) # 

        self.get_logger().info(f'battery level is --> {msg.data:.2f}%')


def main(args=None):

    rclpy.init(args=args)
#
    nod= SensorNode()
    #
    rclpy.spin(nod)

    #
    nod.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    print('~~~starting !!!')
    main()
    print('ending~~~')