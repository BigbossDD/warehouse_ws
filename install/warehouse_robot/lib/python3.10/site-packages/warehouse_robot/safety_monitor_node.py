import rclpy
from rclpy.node import Node
from rcl_interfaces.msg import SetParametersResult
import random

class SafetyMonitorNode(Node):

    def __init__(self):
        super().__init__('safety_monitor_node')

  
        # declare param
        self.declare_parameter('robot_speed', 1.0)
        self.declare_parameter('emergency_stop_dist', 4.0) # NOTE i made it 4 which always trigger 
        #Too close stop the robot , and that to check that the intal parm where taken form
        # the yaml file than from here 
        

        # Read val
        self.robot_speed = self.get_parameter('robot_speed').value
        self.stop_dist = self.get_parameter('emergency_stop_dist').value

        # NOTE Timer to simulate monitoring
        self.timer = self.create_timer(1.0, self.monitor_callback)

        self.get_logger().info("<Safety Monitor Node started>")

       
        self.add_on_set_parameters_callback(self.parameter_callback)#  dynamic updates
################################################################################
    # monitoring section where it will tell to halt or things are safe 
    def monitor_callback(self):

        
        obstacle_distance = random.uniform(0.5, 3.0)#NOTE --> Simulated an obstacle distance
        

        self.get_logger().info(
            f"<SAFETY MONITOR> Obstacle distance: {obstacle_distance:.2f} m | "
            f"Speed: {self.robot_speed} | StopDist: {self.stop_dist}"
        )

        if obstacle_distance < self.stop_dist:
            self.get_logger().warn("<SAFETY MONITOR> TOO CLOSE → STOP ROBOT!")
        else:
            self.get_logger().info("<SAFETY MONITOR> Safe")
###############################
    # Dynamic  Update parm
    
    def parameter_callback(self, params):

        for param in params:
            if param.name == 'robot_speed':
                self.robot_speed = param.value
                self.get_logger().info(f"<SAFETY MONITOR>⚡ Updated speed → {self.robot_speed}")

            elif param.name == 'emergency_stop_dist':
                self.stop_dist = param.value
                self.get_logger().info(f"<SAFETY MONITOR>Updated stop distance → {self.stop_dist}")
        
        return SetParametersResult(successful=True)

##############################################################################3
def main(args=None):
    rclpy.init(args=args)

    node = SafetyMonitorNode()
    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()


if __name__ == '__main__':
    main()